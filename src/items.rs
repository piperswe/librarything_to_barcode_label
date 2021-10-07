use std::{error::Error, fs::File, path::Path};

use barcoders::{generators::svg::SVG, sym::code39::Code39};
use csv::ReaderBuilder;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use eyre::Result;
use serde::Deserialize;

use crate::{CallNumber, Opt};

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(alias = "Barcode")]
    pub barcode: String,
    #[serde(alias = "LC Classification")]
    pub lc_classification: String,
    #[serde(alias = "Dewey Decimal")]
    pub dewey_decimal: String,
}

impl Item {
    pub fn call_number(&self, opt: &Opt) -> String {
        match opt.call_number {
            CallNumber::LCC => process_lcc(&self.lc_classification),
            CallNumber::Dewey => self.dewey_decimal.clone(),
        }
    }

    pub fn render_barcode(&self) -> Result<String> {
        let barcode = Code39::new(&self.barcode)?;
        let svg = SVG::new(32);
        let encoded = barcode.encode();
        Ok(svg.generate(&encoded)?)
    }
}

pub fn read_items(from: &Path) -> Result<impl Iterator<Item = Result<Item, impl Error>>> {
    let rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(File::open(from)?),
        );
    Ok(rdr.into_deserialize())
}

pub fn process_lcc(lcc: &str) -> String {
    let mut s = String::with_capacity(lcc.len());
    let mut last_last: Option<char> = None;
    let mut last: Option<char> = None;
    let mut since_last_zwsp = 0;
    for ch in lcc.chars() {
        if let Some(last) = last {
            if ((ch.is_alphabetic() && last.is_numeric())
                || (ch.is_numeric() && last.is_alphabetic())
                || ch == '.')
                && last != '.'
                && last_last != Some('.')
            {
                s.push('\u{200B}');
                since_last_zwsp = 0;
            }
        }
        if ch.is_whitespace() {
            since_last_zwsp = 0;
        }
        if since_last_zwsp >= 5 {
            s.push('\u{200B}');
            since_last_zwsp = 0;
        }
        since_last_zwsp += 1;
        last_last = last;
        last = Some(ch);
        s.push(ch);
    }
    s
}

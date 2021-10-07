use askama::Template;
use barcoders::generators::svg::*;
use barcoders::sym::code39::*;
use eyre::Result;

use crate::items::Item;
use crate::Opt;

#[derive(Template)]
#[template(path = "labels.html")]
struct LabelsTemplate<'a> {
    items: &'a [Item],
    opt: &'a Opt,
}

pub fn render_page(items: &[Item], opt: &Opt) -> Result<String> {
    let tmpl = LabelsTemplate { items, opt };
    Ok(tmpl.render()?)
}

pub fn render_barcode(number: &str) -> Result<String> {
    let barcode = Code39::new(number)?;
    let svg = SVG::new(32);
    let encoded = barcode.encode();
    Ok(svg.generate(&encoded)?)
}

use std::{fs::File, io::Write, path::PathBuf};

use clap::arg_enum;
use eyre::Result;
use structopt::StructOpt;

mod items;
mod renderer;

arg_enum! {
    #[derive(Debug)]
    pub enum CallNumber {
        LCC,
        Dewey
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "librarything_to_barcode_label",
    about = "Generate barcode labels from a LibraryThing collection"
)]
pub struct Opt {
    #[structopt(
        short, long,
        possible_values = &CallNumber::variants(),
        case_insensitive = true,
        default_value = "LCC"
    )]
    call_number: CallNumber,
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(short, long)]
    start: Option<usize>,
    #[structopt(short, long)]
    end: Option<usize>,
    #[structopt(short, long, default_value = "JetBrains Mono")]
    font: String,
}

impl Opt {
    pub fn font_url(&self) -> String {
        let replaced = self.font.replace(" ", "+");
        format!(
            "https://fonts.googleapis.com/css2?family={}&display=swap",
            replaced
        )
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args_safe()?;
    let item_iter = items::read_items(&opt.input)?;
    let mut items = vec![];
    for item in item_iter {
        let item = item?;
        if let Some(start) = opt.start {
            if item.barcode.parse::<usize>()? < start {
                continue;
            }
        }
        if let Some(end) = opt.end {
            if item.barcode.parse::<usize>()? > end {
                continue;
            }
        }
        if item.collections.contains("Coming Soon") {
            continue;
        }
        items.push(item);
    }
    let rendered = renderer::render_page(&items, &opt)?;
    if let Some(output) = &opt.output {
        let mut file = File::create(output)?;
        file.write_all(rendered.as_bytes())?;
    } else {
        print!("{}", rendered);
    }
    Ok(())
}

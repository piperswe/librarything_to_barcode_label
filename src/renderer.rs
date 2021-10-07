use askama::Template;
use eyre::Result;

use crate::items::Item;
use crate::Opt;

#[derive(Template)]
#[template(path = "labels.html")]
struct LabelsTemplate<'a> {
    items: &'a [Item],
    opt: &'a Opt,
}

pub fn render_page(items: impl Iterator<Item = Item>, opt: &Opt) -> Result<String> {
    let tmpl = LabelsTemplate {
        items: &items.collect::<Vec<Item>>(),
        opt,
    };
    Ok(tmpl.render()?)
}

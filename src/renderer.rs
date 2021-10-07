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

pub fn render_page(items: &[Item], opt: &Opt) -> Result<String> {
    let tmpl = LabelsTemplate { items, opt };
    Ok(tmpl.render()?)
}

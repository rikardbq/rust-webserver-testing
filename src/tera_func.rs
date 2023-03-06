use std::{str};
use serde_json::Value;
use tera::Tera;

const PAGE_DIR: &str = "template/page/*";
const FRAGMENT_DIR: &str = "template/fragment/*";

pub fn mk_page(tmpl: &str, value: &Value) -> String {
    render_tpl(true, tmpl, value)
}

pub fn mk_fragment(tmpl: &str, value: &Value) -> String {
    render_tpl(false, tmpl, value)
}

fn render_tpl(page: bool, tmpl: &str, value: &Value) -> String {
    let mut tera_tpl = Tera::new(if page { PAGE_DIR } else { FRAGMENT_DIR }).unwrap();
    tera_tpl.autoescape_on(vec![]);

    let tera_ctx = tera::Context::from_value(value.to_owned()).unwrap();
    let mut buf = Vec::new();

    _ = tera_tpl.render_to([tmpl, ".html"].join("").as_str(), &tera_ctx, &mut buf);

    String::from_utf8(buf).unwrap()
}

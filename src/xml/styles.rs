use quick_xml::events::BytesStart;

use errors::{Error, Result};
use schema::SCHEMA_MAIN;
use style::Style;
use xml::Xml;

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct StylesXml<'a> {
  #[xml(child)]
  #[xml(tag = "w:style")]
  styles: Vec<Style<'a>>,
}

fn styles_extend_attrs(_: &StylesXml, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

impl<'a> StylesXml<'a> {
  pub fn create_style(&mut self) -> &mut Style<'a> {
    self.styles.push(Style::default());
    self.styles.last_mut().unwrap()
  }
}

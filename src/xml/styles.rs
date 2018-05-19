use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use schema::SCHEMA_MAIN;
use style::Style;
use xml::Xml;

#[derive(Debug)]
pub struct StylesXml<'a> {
  styles: Vec<&'a Style<'a>>,
}

impl<'a> StylesXml<'a> {
  pub fn append_style(&mut self, style: &'a Style<'a>) {
    if !self.styles.iter().any(|&x| x.name == style.name) {
      self.styles.push(style);
    }
  }
}

impl<'a> Default for StylesXml<'a> {
  fn default() -> Self {
    StylesXml { styles: Vec::new() }
  }
}

impl<'a> Xml<'a> for StylesXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:styles", "xmlns:w", SCHEMA_MAIN) {[
      for style in &self.styles {
        style.write(writer)?;
      }
    ]});
    Ok(())
  }
}

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
  styles: Vec<Style<'a>>,
}

impl<'a> StylesXml<'a> {
  pub fn create_style(&mut self) -> &mut Style<'a> {
    self.styles.push(Style::default());
    self.styles.last_mut().unwrap()
  }
}

impl<'a> Default for StylesXml<'a> {
  fn default() -> Self {
    StylesXml { styles: Vec::new() }
  }
}

impl<'a> Xml<'a> for StylesXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:styles"["xmlns:w", SCHEMA_MAIN] {{
      for style in &self.styles {
        style.write(w)?;
      }
    }});
    Ok(())
  }
}

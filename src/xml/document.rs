use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use body::Para;
use errors::Result;
use schema::SCHEMA_MAIN;
use xml::Xml;

#[derive(Debug)]
pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>,
}

impl<'a> DocumentXml<'a> {
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.body.push(Para::default());
    self.body.last_mut().unwrap()
  }
}

impl<'a> Default for DocumentXml<'a> {
  fn default() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }
}

impl<'a> Xml<'a> for DocumentXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"w:document", "xmlns:w", SCHEMA_MAIN) {
      b"w:body" {[
        for para in &self.body {
          para.write(writer)?;
        }
      ]}
    });
    Ok(())
  }
}

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
  pub fn add_para(&mut self, para: Para<'a>) {
    self.body.push(para);
  }
}

impl<'a> Default for DocumentXml<'a> {
  fn default() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }
}

impl<'a> Xml<'a> for DocumentXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_start_event!(writer, b"w:document", "xmlns:w", SCHEMA_MAIN);
    write_start_event!(writer, b"w:body");
    for para in &self.body {
      para.write(writer)?;
    }
    write_end_event!(writer, b"w:body");
    write_end_event!(writer, b"w:document");
    Ok(())
  }
}

use quick_xml::events::*;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use body::Para;
use errors::Result;
use schema::SCHEMA_MAIN;
use xml::Xml;

#[derive(Debug, Default)]
pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>,
}

impl<'a> DocumentXml<'a> {
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.body.push(Para::default());
    self.body.last_mut().unwrap()
  }
}

impl<'a> Xml for DocumentXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"w:document"["xmlns:w", SCHEMA_MAIN] {{
      tag!(w, b"w:body" {{
        for para in &self.body {
          para.write(w)?;
        }
      }});
    }});
    Ok(())
  }
}

use quick_xml::events::*;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use schema::SCHEMA_CORE;
use xml::Xml;

#[derive(Debug, Default)]
pub struct CoreXml<'a> {
  title: &'a str,
  subject: &'a str,
  creator: &'a str,
  keywords: &'a str,
  description: &'a str,
  last_modified_by: &'a str,
  revision: &'a str,
}

impl<'a> Xml<'a> for CoreXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"cp:coreProperties"["xmlns:cp", SCHEMA_CORE] {{
      tag!(w, b"dc:title"{self.title});
      tag!(w, b"dc:subject"{self.subject});
      tag!(w, b"dc:creator"{self.creator});
      tag!(w, b"cp:keywords"{self.keywords});
      tag!(w, b"dc:description"{self.description});
      tag!(w, b"cp:lastModifiedBy"{self.last_modified_by});
      tag!(w, b"cp:revision"{self.revision});
      // TODO: <dcterms:created> and <dcterms:modified>
    }});
    Ok(())
  }
}

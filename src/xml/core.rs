use quick_xml::events::*;
use quick_xml::Result;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use schema::SCHEMA_CORE;
use xml::Xml;

#[derive(Debug)]
pub struct CoreXml<'a> {
  title: &'a str,
  subject: &'a str,
  creator: &'a str,
  keywords: &'a str,
  description: &'a str,
  last_modified_by: &'a str,
  revision: &'a str,
}

impl<'a> Default for CoreXml<'a> {
  fn default() -> CoreXml<'a> {
    CoreXml {
      title: "",
      subject: "",
      creator: "",
      keywords: "",
      description: "",
      last_modified_by: "",
      revision: "",
    }
  }
}

impl<'a> Xml<'a> for CoreXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"cp:coreProperties", "xmlns:cp", SCHEMA_CORE) {
      b"dc:title"{self.title}
      b"dc:subject"{self.subject}
      b"dc:creator"{self.creator}
      b"cp:keywords"{self.keywords}
      b"dc:description"{self.description}
      b"cp:lastModifiedBy"{self.last_modified_by}
      b"cp:revision"{self.revision}
      // TODO: <dcterms:created> and <dcterms:modified>
    });
    Ok(())
  }
}

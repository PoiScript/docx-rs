use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use content_type::{
  CONTENT_TYPE_CORE, CONTENT_TYPE_DOCUMENT, CONTENT_TYPE_EXTENDED, CONTENT_TYPE_RELATIONSHIP,
  CONTENT_TYPE_XML,
};
use errors::Result;
use schema::SCHEMA_CONTENT_TYPES;
use xml::Xml;

static DEFAULTS_CT: [(&str, &str); 2] = [
  ("rels", CONTENT_TYPE_RELATIONSHIP),
  ("xml", CONTENT_TYPE_XML),
];

static OVERRIDES_CT: [(&str, &str); 3] = [
  ("/docProps/app.xml", CONTENT_TYPE_EXTENDED),
  ("/docProps/core.xml", CONTENT_TYPE_CORE),
  ("/word/document.xml", CONTENT_TYPE_DOCUMENT),
];

#[derive(Debug)]
pub struct ContentTypesXml<'a> {
  defaults: Vec<(&'a str, &'a str)>,
  overrides: Vec<(&'a str, &'a str)>,
}

impl<'a> Default for ContentTypesXml<'a> {
  fn default() -> ContentTypesXml<'a> {
    ContentTypesXml {
      defaults: DEFAULTS_CT.to_vec(),
      overrides: OVERRIDES_CT.to_vec(),
    }
  }
}

impl<'a> Xml<'a> for ContentTypesXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (b"Types", "xmlns", SCHEMA_CONTENT_TYPES) {
      [
        for &(extension, content_type) in &self.defaults {
          write_empty_event!(
            writer,
            b"Default",
            "Extension",
            extension,
            "ContentType",
            content_type
          );
        }
      ]
      [
        for &(part_name, content_type) in &self.overrides {
          write_empty_event!(
            writer,
            b"Override",
            "PartName",
            part_name,
            "ContentType",
            content_type
          );
        }
      ]
    });
    Ok(())
  }
}

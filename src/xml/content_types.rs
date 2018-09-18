use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use content_type::{
  CONTENT_TYPE_CORE, CONTENT_TYPE_DOCUMENT, CONTENT_TYPE_EXTENDED, CONTENT_TYPE_RELATIONSHIP,
  CONTENT_TYPE_STYLES, CONTENT_TYPE_XML,
};
use errors::Result;
use schema::SCHEMA_CONTENT_TYPES;
use xml::Xml;

static DEFAULTS_CT: [(&str, &str); 2] = [
  ("rels", CONTENT_TYPE_RELATIONSHIP),
  ("xml", CONTENT_TYPE_XML),
];

static OVERRIDES_CT: [(&str, &str); 4] = [
  ("/docProps/app.xml", CONTENT_TYPE_EXTENDED),
  ("/docProps/core.xml", CONTENT_TYPE_CORE),
  ("/word/document.xml", CONTENT_TYPE_DOCUMENT),
  ("/word/styles.xml", CONTENT_TYPE_STYLES),
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

impl<'a> Xml for ContentTypesXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"Types"["xmlns", SCHEMA_CONTENT_TYPES] {{
      for &(extension, content_type) in &self.defaults {
        tag!(w, b"Default"[
          "Extension",
          extension,
          "ContentType",
          content_type
        ]);
      }
      for &(part_name, content_type) in &self.overrides {
        tag!(w, b"Override"[
          "PartName",
          part_name,
          "ContentType",
          content_type
        ]);
      }
    }});
    Ok(())
  }
}

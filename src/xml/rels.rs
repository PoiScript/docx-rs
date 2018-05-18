use quick_xml::events::*;
use quick_xml::Result;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use schema::{
  SCHEMA_CORE, SCHEMA_FONT_TABLE, SCHEMA_OFFICE_DOCUMENT, SCHEMA_RELATIONSHIPS,
  SCHEMA_REL_EXTENDED, SCHEMA_SETTINGS, SCHEMA_STYLES,
};
use xml::Xml;

#[derive(Debug)]
pub struct RelsXml<'a> {
  relationships: Vec<(&'a str, &'a str)>,
}

impl<'a> RelsXml<'a> {
  /// Return default relationships for document.xml
  pub fn document() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (SCHEMA_STYLES, "styles.xml"),
        (SCHEMA_FONT_TABLE, "fontTable.xml"),
        (SCHEMA_SETTINGS, "settings.xml"),
      ],
    }
  }

  pub fn add_rel(&mut self, rel: (&'a str, &'a str)) {
    self.relationships.push(rel);
  }
}

impl<'a> Default for RelsXml<'a> {
  /// Return default relationships for the whole package
  fn default() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![(SCHEMA_OFFICE_DOCUMENT, "word/document.xml")],
    }
  }
}

impl<'a> Xml<'a> for RelsXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_start_event!(writer, b"Relationships", "xmlns", SCHEMA_RELATIONSHIPS);
    for (i, (schema, target)) in self.relationships.iter().enumerate() {
      write_empty_event!(
        writer,
        b"Relationship",
        "Id",
        format!("rId{}", i).as_str(),
        "Target",
        *target,
        "Type",
        *schema
      );
    }
    write_end_event!(writer, b"Relationships");
    Ok(())
  }
}

use quick_xml::events::*;
use quick_xml::Writer;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
use schema::SCHEMA_RELATIONSHIPS;
use xml::Xml;

#[derive(Debug, Default)]
pub struct RelsXml<'a> {
  relationships: Vec<(&'a str, &'a str)>,
}

impl<'a> RelsXml<'a> {
  pub fn add_rel(&mut self, schema: &'a str, target: &'a str) {
    self.relationships.push((schema, target));
  }
}

impl<'a> Xml<'a> for RelsXml<'a> {
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"Relationships"["xmlns", SCHEMA_RELATIONSHIPS] {{
      for (i, (schema, target)) in self.relationships.iter().enumerate() {
        tag!(w, b"Relationship" [
          "Id",
          format!("rId{}", i).as_str(),
          "Target",
          *target,
          "Type",
          *schema
        ]);
      }
    }});
    Ok(())
  }
}

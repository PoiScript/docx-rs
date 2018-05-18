use quick_xml::events::*;
use quick_xml::Result;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};
use xml::Xml;

#[derive(Debug)]
pub struct AppXml<'a> {
  template: &'a str,
  total_time: &'a str,
  pages: &'a str,
  words: &'a str,
  characters: &'a str,
  applications: &'a str,
  doc_security: &'a str,
  lines: &'a str,
  paragraphs: &'a str,
  scale_crop: &'a str,
  company: &'a str,
  links_up_to_date: &'a str,
  characters_with_spaces: &'a str,
  shared_doc: &'a str,
  hyperlinks_changed: &'a str,
  app_version: &'a str,
}

impl<'a> Default for AppXml<'a> {
  fn default() -> AppXml<'a> {
    AppXml {
      template: "Normal.dotm",
      total_time: "1",
      pages: "1",
      words: "0",
      characters: "0",
      applications: "docx-rs",
      doc_security: "0",
      lines: "0",
      paragraphs: "1",
      scale_crop: "false",
      company: "MS",
      links_up_to_date: "false",
      characters_with_spaces: "25",
      shared_doc: "false",
      hyperlinks_changed: "false",
      app_version: "12.0000",
    }
  }
}

impl<'a> Xml<'a> for AppXml<'a> {
  fn write<T: Write + Seek>(&self, writer: &mut Writer<ZipWriter<T>>) -> Result<()> {
    write_events!(writer, (
      b"Properties",
      "xmlns",
      SCHEMAS_EXTENDED,
      "xmlns:vt",
      SCHEMA_DOC_PROPS_V_TYPES
    ) {
      b"SharedDoc"{self.shared_doc}
      b"Template"{self.template}
      b"TotalTime"{self.total_time}
      b"Pages"{self.pages}
      b"Words"{self.words}
      b"Characters"{self.characters}
      b"Application"{self.applications}
      b"DocSecurity"{self.doc_security}
      b"Lines"{self.lines}
      b"Paragraphs"{self.paragraphs}
      b"ScaleCrop"{self.scale_crop}
      b"Company"{self.company}
      b"LinksUpToDate"{self.links_up_to_date}
      b"CharactersWithSpaces"{self.characters_with_spaces}
      b"SharedDoc"{self.shared_doc}
      b"HyperlinksChanged"{self.hyperlinks_changed}
      b"AppVersion"{self.app_version}
    });
    Ok(())
  }
}

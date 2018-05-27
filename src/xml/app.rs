use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::ZipWriter;

use errors::Result;
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
  fn write<T: Write + Seek>(&self, w: &mut Writer<ZipWriter<T>>) -> Result<()> {
    tag!(w, b"Properties" ["xmlns",SCHEMAS_EXTENDED,"xmlns:vt",SCHEMA_DOC_PROPS_V_TYPES] {{
      tag!(w, b"SharedDoc"{self.shared_doc});
      tag!(w, b"Template"{self.template});
      tag!(w, b"TotalTime"{self.total_time});
      tag!(w, b"Pages"{self.pages});
      tag!(w, b"Words"{self.words});
      tag!(w, b"Characters"{self.characters});
      tag!(w, b"Application"{self.applications});
      tag!(w, b"DocSecurity"{self.doc_security});
      tag!(w, b"Lines"{self.lines});
      tag!(w, b"Paragraphs"{self.paragraphs});
      tag!(w, b"ScaleCrop"{self.scale_crop});
      tag!(w, b"Company"{self.company});
      tag!(w, b"LinksUpToDate"{self.links_up_to_date});
      tag!(w, b"CharactersWithSpaces"{self.characters_with_spaces});
      tag!(w, b"SharedDoc"{self.shared_doc});
      tag!(w, b"HyperlinksChanged"{self.hyperlinks_changed});
      tag!(w, b"AppVersion"{self.app_version});
    }});
    Ok(())
  }
}

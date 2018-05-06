/// docProps/app.xml

use std::collections::LinkedList;
use std::io::Cursor;
use quick_xml::events::*;
use quick_xml::Writer;

use utility::LinkUtil;

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

impl<'a> AppXml<'a> {
  pub fn default() -> AppXml<'a> {
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

  pub fn generate(&self) -> Vec<u8> {
    let mut events = LinkedList::new();

    events
      .add_tag(b"Template", self.template)
      .add_tag(b"Template", self.template)
      .add_tag(b"TotalTime", self.total_time)
      .add_tag(b"Pages", self.pages)
      .add_tag(b"Words", self.words)
      .add_tag(b"Characters", self.characters)
      .add_tag(b"Application", self.applications)
      .add_tag(b"DocSecurity", self.doc_security)
      .add_tag(b"Lines", self.lines)
      .add_tag(b"Paragraphs", self.paragraphs)
      .add_tag(b"ScaleCrop", self.scale_crop)
      .add_tag(b"Company", self.company)
      .add_tag(b"LinksUpToDate", self.links_up_to_date)
      .add_tag(b"CharactersWithSpaces", self.characters_with_spaces)
      .add_tag(b"SharedDoc", self.shared_doc)
      .add_tag(b"HyperlinksChanged", self.hyperlinks_changed)
      .add_tag(b"AppVersion", self.app_version)
      .wrap_tag_with_attr(b"Properties", vec![
        ("xmlns", "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"),
        ("xmlns:vt", "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes")
      ])
      .add_decl()
      .to_xml()
  }
}

use quick_xml::events::Event;
use std::collections::LinkedList;

use events_list::EventListExt;
use xml::Xml;

static PROPERTIES_NAMESPACES: [(&'static str, &'static str); 2] = [
  (
    "xmlns",
    "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
  ),
  (
    "xmlns:vt",
    "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
  ),
];

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

impl<'a> Xml<'a> for AppXml<'a> {
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

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    events
      .add_text_tag("SharedDoc", self.shared_doc)
      .add_text_tag("Template", self.template)
      .add_text_tag("Template", self.template)
      .add_text_tag("TotalTime", self.total_time)
      .add_text_tag("Pages", self.pages)
      .add_text_tag("Words", self.words)
      .add_text_tag("Characters", self.characters)
      .add_text_tag("Application", self.applications)
      .add_text_tag("DocSecurity", self.doc_security)
      .add_text_tag("Lines", self.lines)
      .add_text_tag("Paragraphs", self.paragraphs)
      .add_text_tag("ScaleCrop", self.scale_crop)
      .add_text_tag("Company", self.company)
      .add_text_tag("LinksUpToDate", self.links_up_to_date)
      .add_text_tag("CharactersWithSpaces", self.characters_with_spaces)
      .add_text_tag("SharedDoc", self.shared_doc)
      .add_text_tag("HyperlinksChanged", self.hyperlinks_changed)
      .add_text_tag("AppVersion", self.app_version)
      .warp_attrs_tag("Properties", PROPERTIES_NAMESPACES.to_vec());

    events
  }
}

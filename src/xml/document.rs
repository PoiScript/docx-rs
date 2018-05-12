use quick_xml::events::Event;
use std::collections::LinkedList;

use body::Para;
use events_list::EventListExt;
use schema::SCHEMA_MAIN;
use xml::Xml;

static DOCUMENT_NAMESPACES: [(&'static str, &'static str); 1] = [("xmlns:w", SCHEMA_MAIN)];

pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>,
}

impl<'a> DocumentXml<'a> {
  pub fn add_para(&mut self, para: Para<'a>) {
    self.body.push(para);
  }
}

impl<'a> Xml<'a> for DocumentXml<'a> {
  fn default() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for para in &self.body {
      events.append(&mut para.events());
    }

    events
      .warp_tag("w:body")
      .warp_attrs_tag("w:document", DOCUMENT_NAMESPACES.to_vec());

    events
  }
}

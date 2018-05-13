use quick_xml::events::*;
use std::collections::LinkedList;

use content_type::{
  CONTENT_TYPE_CORE, CONTENT_TYPE_DOCUMENT, CONTENT_TYPE_EXTENDED, CONTENT_TYPE_RELATIONSHIP,
  CONTENT_TYPE_XML,
};
use events_list::EventListExt;
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

static CONTENT_TYPES_NAMESPACES: [(&'static str, &'static str); 1] =
  [("xmlns", SCHEMA_CONTENT_TYPES)];

#[derive(Debug)]
pub struct ContentTypesXml<'a> {
  defaults: Vec<(&'a str, &'a str)>,
  overrides: Vec<(&'a str, &'a str)>,
}

impl<'a> Xml<'a> for ContentTypesXml<'a> {
  fn default() -> ContentTypesXml<'a> {
    ContentTypesXml {
      defaults: DEFAULTS_CT.to_vec(),
      overrides: OVERRIDES_CT.to_vec(),
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for &(extension, content_type) in &self.defaults {
      events.add_attrs_empty_tag(
        "Default",
        vec![("Extension", extension), ("ContentType", content_type)],
      );
    }

    for &(part_name, content_type) in &self.overrides {
      events.add_attrs_empty_tag(
        "Override",
        vec![("PartName", part_name), ("ContentType", content_type)],
      );
    }

    events.warp_attrs_tag("Types", CONTENT_TYPES_NAMESPACES.to_vec());

    events
  }
}

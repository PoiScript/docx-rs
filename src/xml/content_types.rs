use quick_xml::events::*;
use std::collections::LinkedList;

use events_list::EventListExt;
use xml::Xml;

static DEFAULTS_CT: &[(&'static str, &'static str); 2] = &[
  (
    "rels",
    "application/vnd.openxmlformats-package.relationships+xml",
  ),
  ("xml", "application/xml"),
];

static OVERRIDES_CT: [(&'static str, &'static str); 3] = [
  (
    "/docProps/app.xml",
    "application/vnd.openxmlformats-officedocument.extended-properties+xml",
  ),
  (
    "/docProps/core.xml",
    "application/vnd.openxmlformats-package.core-properties+xml",
  ),
  (
    "/word/document.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
  ),
];

static CONTENT_TYPES_NAMESPACES: [(&'static str, &'static str); 1] = [(
  "xmlns",
  "http://schemas.openxmlformats.org/package/2006/content-types",
)];

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

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

static OVERRIDES_CT: [(&'static str, &'static str); 8] = [
  (
    "/word/document.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
  ),
  (
    "/word/styles.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
  ),
  (
    "/docProps/app.xml",
    "application/vnd.openxmlformats-officedocument.extended-properties+xml",
  ),
  (
    "/word/settings.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
  ),
  (
    "/word/theme/theme1.xml",
    "application/vnd.openxmlformats-officedocument.theme+xml",
  ),
  (
    "/word/fontTable.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
  ),
  (
    "/word/webSettings.xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
  ),
  (
    "/docProps/core.xml",
    "application/vnd.openxmlformats-package.core-properties+xml",
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

    let mut iter = self.defaults.iter();
    while let Some(&(part_name, content_type)) = iter.next() {
      events.push_back(Event::Empty(
        BytesStart::borrowed(b"Default", b"Default".len())
          .with_attributes(vec![("PartName", part_name), ("ContentType", content_type)]),
      ));
    }

    let mut iter = self.overrides.iter();
    while let Some(&(part_name, content_type)) = iter.next() {
      events.push_back(Event::Empty(
        BytesStart::borrowed(b"Override", b"Override".len())
          .with_attributes(vec![("PartName", part_name), ("ContentType", content_type)]),
      ));
    }

    events.warp_attrs_tag("Types", CONTENT_TYPES_NAMESPACES.to_vec());

    events
  }
}

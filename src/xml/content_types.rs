use quick_xml::events::Event;
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

    // TODO
    //    for i in 0..self.defaults.len() {
    //      let vec = self
    //        .defaults
    //        .into_iter()
    //        .map(|(part_name, content_type)| {
    //          vec![("PartName", part_name), ("ContentType", content_type)]
    //        })
    //        .collect::<Vec<(&str, &str)>>();
    //      events.add_attrs_empty_tag("Default", vec);
    //    }
    //    for (part_name, content_type) in self.defaults.iter() {
    //      events.add_attrs_empty_tag(
    //        "Default",
    //        vec![("PartName", part_name), ("ContentType", content_type)],
    //      );
    //    }
    //
    //    for i in 0..self.overrides.len() {
    //      let (part_name, content_type) = self.overrides[i];
    //      events.add_attrs_empty_tag(
    //        "Override",
    //        vec![("PartName", part_name), ("ContentType", content_type)],
    //      );
    //    }

    events.warp_attrs_tag("Types", CONTENT_TYPES_NAMESPACES.to_vec());

    events
  }
}

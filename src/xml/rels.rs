use quick_xml::events::*;
use std::collections::LinkedList;

use events_list::EventListExt;
use schema::{
  SCHEMA_CORE, SCHEMA_FONT_TABLE, SCHEMA_OFFICE_DOCUMENT, SCHEMA_RELATIONSHIPS,
  SCHEMA_REL_EXTENDED, SCHEMA_SETTINGS, SCHEMA_STYLES,
};
use xml::Xml;

static RELATIONSHIPS_NAMESPACES: [(&'static str, &'static str); 1] =
  [("xmlns", SCHEMA_RELATIONSHIPS)];

pub struct RelsXml<'a> {
  relationships: Vec<(&'static str, &'a str)>,
}

impl<'a> RelsXml<'a> {
  /// Return default relationships for document.xml
  pub fn document() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (SCHEMA_STYLES, "styles.xml"),
        (SCHEMA_FONT_TABLE, "fontTable.xml"),
        (SCHEMA_SETTINGS, "settings.xml"),
      ],
    }
  }

  pub fn add_rel(&mut self, rel: (&'static str, &'a str)) {
    self.relationships.push(rel);
  }
}

impl<'a> Xml<'a> for RelsXml<'a> {
  /// Return default relationships for the whole package
  fn default() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (SCHEMA_CORE, "docProps/core.xml"),
        (SCHEMA_REL_EXTENDED, "docProps/app.xml"),
        (SCHEMA_OFFICE_DOCUMENT, "word/document.xml"),
      ],
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for (i, (schema, target)) in self.relationships.iter().enumerate() {
      events.push_back(Event::Empty(
        BytesStart::borrowed(b"Relationship", b"Relationship".len()).with_attributes(vec![
          ("Id", format!("rId{}", i).as_str()),
          ("Target", target),
          ("Type", schema),
        ]),
      ));
    }

    events.warp_attrs_tag("Relationships", RELATIONSHIPS_NAMESPACES.to_vec());

    events
  }
}

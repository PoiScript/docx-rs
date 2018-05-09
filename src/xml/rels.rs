use quick_xml::events::*;
use std::collections::LinkedList;

use events_list::EventListExt;
use xml::Xml;

static RELATIONSHIPS_NAMESPACES: [(&'static str, &'static str); 1] = [(
  "xmlns",
  "http://schemas.openxmlformats.org/package/2006/relationships",
)];

static OFFICE_DOCUMENT_SCHEMAS: &'static str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
static CORE_SCHEMAS: &'static str =
  "http://schemas.openxmlformats.org/officedocument/2006/relationships/metadata/core-properties";
static EXTENDED_SCHEMAS: &'static str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";

pub enum Rel {
  Core,
  Extended,
  Document,
  Theme,
  Settings,
  FontTable,
  Styles,
  Image,
  Header,
  Numbering,
}

pub struct RelsXml<'a> {
  relationships: Vec<(Rel, &'a str)>,
}

impl<'a> RelsXml<'a> {
  pub fn add_rel(&mut self, rel: (Rel, &'a str)) {
    self.relationships.push(rel);
  }
}

impl<'a> Xml<'a> for RelsXml<'a> {
  fn default() -> RelsXml<'a> {
    RelsXml {
      relationships: vec![
        (Rel::Core, "docProps/core.xml"),
        (Rel::Extended, "docProps/app.xml"),
        (Rel::Document, "word/document.xml"),
      ],
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    let mut iter = self.relationships.iter().enumerate();
    while let Some((i, &(ref rel_type, target))) = iter.next() {
      events.push_back(Event::Empty(
        BytesStart::borrowed(b"Relationship", b"Relationship".len()).with_attributes(vec![
          ("Id", format!("rId{}", i).as_str()),
          ("Target", target),
          (
            "Type",
            match rel_type {
              &Rel::Document => OFFICE_DOCUMENT_SCHEMAS,
              &Rel::Core => CORE_SCHEMAS,
              &Rel::Extended => EXTENDED_SCHEMAS,
              // TODO: more schemas
              _ => CORE_SCHEMAS,
            },
          ),
        ]),
      ));
    }

    events.warp_attrs_tag("Relationships", RELATIONSHIPS_NAMESPACES.to_vec());

    events
  }
}

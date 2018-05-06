use std::collections::LinkedList;
use std::slice;

use utility::LinkUtil;

static RELATIONSHIPS_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/package/2006/relationships";
static OFFICE_DOCUMENT_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
static CORE_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/officedocument/2006/relationships/metadata/core-properties";
static EXTENDED_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";

// FIXME
static IDS: [&'static str; 14] = [
  "rId1",
  "rId2",
  "rId3",
  "rId4",
  "rId5",
  "rId6",
  "rId7",
  "rId8",
  "rId9",
  "rId10",
  "rId11",
  "rId12",
  "rId13",
  "rId14",
];

pub enum Relationship {
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

pub struct Relationships<'a> {
  relationships: Vec<(Relationship, &'a str)>,
}

impl<'a> Relationships<'a> {
  pub fn default() -> Relationships<'a> {
    Relationships {
      relationships: vec![
        (Relationship::Core, "docProps/core.xml"),
        (Relationship::Extended, "docProps/app.xml"),
        (Relationship::Document, "word/document.xml"),
      ]
    }
  }

  pub fn add_rel(&mut self, rel: (Relationship, &'a str)) {
    self.relationships.push(rel);
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = LinkedList::new();

    for i in 0..self.relationships.len() {
      let (ref rel_type, target) = self.relationships[i];
      events.add_tag_with_attr(b"Relationship", vec![
        ("Id", IDS[i]),
        ("Target", target),
        ("Type", match rel_type {
          &Relationship::Document => OFFICE_DOCUMENT_SCHEMAS,
          &Relationship::Core => CORE_SCHEMAS,
          &Relationship::Extended => EXTENDED_SCHEMAS,
          // TODO: more schemas
          _ => CORE_SCHEMAS
        })
      ]);
    }

    events
      .wrap_tag_with_attr(b"Relationships", vec![
        ("xmlns", RELATIONSHIPS_SCHEMAS)
      ])
      .add_decl()
      .to_xml()
  }
}

use std::collections::LinkedList;
use std::slice;

use utility::LinkUtil;

static RELATIONSHIPS_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/package/2006/relationships";
static OFFICE_DOCUMENT_SCHEMAS: &'static str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

pub enum Relationship<'a> {
  Document(&'a str),
  Theme(&'a str),
  Settings(&'a str),
  FontTable(&'a str),
  Styles(&'a str),
  Image(&'a str),
  Header(&'a str),
  Numbering(&'a str),
}

pub struct Relationships<'a> {
  relationships: Vec<Relationship<'a>>,
}

impl<'a> Relationships<'a> {
  pub fn default() -> Relationships<'a> {
    Relationships { relationships: vec![Relationship::Document("word/document.xml")] }
  }

  pub fn add_rel(&mut self, rel: Relationship<'a>) {
    self.relationships.push(rel);
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = LinkedList::new();

    for i in 0..self.relationships.len() {
      match self.relationships[i] {
        Relationship::Document(target) => {
          events.add_tag_with_attr(b"Relationship", vec![
            // TODO: add relationship id
            ("Type", OFFICE_DOCUMENT_SCHEMAS),
            ("Target", target)
          ]);
        }
        _ => ()
      }
    }

    events
      .wrap_tag_with_attr(b"Relationships", vec![
        ("xmlns", RELATIONSHIPS_SCHEMAS)
      ])
      .add_decl()
      .to_xml()
  }
}

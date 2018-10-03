//! Relationship item
//!
//! The corresponding ZIP item is `/_rels/.rels` (package-relationship) or `/word/_rels/document.xml.rels` (part-relationship).

use quick_xml::events::BytesStart;
use std::borrow::Borrow;
use std::borrow::Cow;

use errors::{Error, Result};
use schema::SCHEMA_RELATIONSHIPS;

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "Relationships")]
#[xml(extend_attrs = "relationships_extend_attrs")]
pub struct Relationships<'a> {
  #[xml(child)]
  #[xml(tag = "Relationship")]
  pub relationships: Vec<Relationship<'a>>,
}

#[inline]
fn relationships_extend_attrs(_: &Relationships, start: &mut BytesStart) {
  start.push_attribute(("xmlns", SCHEMA_RELATIONSHIPS));
}

impl<'a> Relationships<'a> {
  pub fn add_rel(&mut self, schema: &'a str, target: &'a str) {
    let len = self.relationships.len();
    self.relationships.push(Relationship {
      id: Cow::Owned(format!("rId{}", len)),
      target: Cow::Borrowed(target),
      ty: Cow::Borrowed(schema),
    });
  }

  pub fn get_target(&'a self, id: &'a str) -> Option<&'a str> {
    self
      .relationships
      .iter()
      .find(|r| r.id == id)
      .map(|r| r.target.borrow())
  }
}

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "Relationship")]
pub struct Relationship<'a> {
  #[xml(attr = "Id")]
  pub id: Cow<'a, str>,
  #[xml(attr = "Target")]
  pub target: Cow<'a, str>,
  #[xml(attr = "Type")]
  pub ty: Cow<'a, str>,
}

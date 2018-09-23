use quick_xml::events::BytesStart;
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
  relationships: Vec<Relationship<'a>>,
}

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
}

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "Relationship")]
struct Relationship<'a> {
  #[xml(attr = "Id")]
  id: Cow<'a, str>,
  #[xml(attr = "Target")]
  target: Cow<'a, str>,
  #[xml(attr = "Type")]
  ty: Cow<'a, str>,
}

//! Relationship item
//!
//! The corresponding ZIP item is `/_rels/.rels` (package-relationship) or `/word/_rels/document.xml.rels` (part-relationship).

use crate::errors::{Error, Result};
use crate::schema::SCHEMA_RELATIONSHIPS;
use quick_xml::events::BytesStart;

#[derive(Debug, Default, Xml)]
#[xml(tag = "Relationships")]
#[xml(extend_attrs = "relationships_extend_attrs")]
pub struct Relationships {
    #[xml(child = "Relationship")]
    pub relationships: Vec<Relationship>,
}

#[inline]
fn relationships_extend_attrs(_: &Relationships, start: &mut BytesStart) {
    start.push_attribute(("xmlns", SCHEMA_RELATIONSHIPS));
}

impl Relationships {
    pub fn add_rel(&mut self, schema: &str, target: &str) {
        let len = self.relationships.len();
        self.relationships.push(Relationship {
            id: format!("rId{}", len),
            target: target.to_owned(),
            ty: schema.to_owned(),
        });
    }

    pub fn get_target(&self, id: &str) -> Option<&str> {
        self.relationships
            .iter()
            .find(|r| r.id == id)
            .map(|r| &*r.target)
    }
}

#[derive(Debug, Xml)]
#[xml(tag = "Relationship")]
#[xml(leaf)]
pub struct Relationship {
    #[xml(attr = "Id")]
    pub id: String,
    #[xml(attr = "Target")]
    pub target: String,
    #[xml(attr = "Type")]
    pub ty: String,
}

//! Relationship item
//!
//! The corresponding ZIP item is `/_rels/.rels` (package-relationship) or
//! `/word/_rels/document.xml.rels` (part-relationship).

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

use crate::schema::SCHEMA_RELATIONSHIPS;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "Relationships")]
#[xml(extend_attrs = "relationships_extend_attrs")]
pub struct Relationships<'a> {
    #[xml(child = "Relationship")]
    pub relationships: Vec<Relationship<'a>>,
}

#[inline]
fn relationships_extend_attrs<W: Write>(_: &Relationships, mut w: W) -> XmlResult<()> {
    write!(w, " xmlns=\"{}\"", SCHEMA_RELATIONSHIPS)?;
    Ok(())
}

impl<'a> Relationships<'a> {
    pub fn add_rel(&mut self, schema: &'a str, target: &'a str) {
        let len = self.relationships.len();
        self.relationships.push(Relationship {
            id: format!("rId{}", len).into(),
            target: target.into(),
            ty: schema.into(),
        });
    }

    pub fn get_target(&self, id: &str) -> Option<&str> {
        self.relationships
            .iter()
            .find(|r| r.id == id)
            .map(|r| &*r.target)
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "Relationship")]
pub struct Relationship<'a> {
    #[xml(attr = "Id")]
    pub id: Cow<'a, str>,
    #[xml(attr = "Target")]
    pub target: Cow<'a, str>,
    #[xml(attr = "Type")]
    pub ty: Cow<'a, str>,
}

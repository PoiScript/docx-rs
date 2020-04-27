//! Relationship item
//!
//! The corresponding ZIP item is `/_rels/.rels` (package-relationship) or
//! `/word/_rels/document.xml.rels` (part-relationship).

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::schema::SCHEMA_RELATIONSHIPS;

#[derive(Debug, Default, XmlRead)]
#[xml(tag = "Relationships")]
pub struct Relationships<'a> {
    #[xml(child = "Relationship")]
    pub relationships: Vec<Relationship<'a>>,
}

impl<'a> XmlWrite for Relationships<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Relationships { relationships } = self;

        log::debug!("[Relationships] Started writing.");

        writer.write_element_start("Relationships")?;

        writer.write_attribute("xmlns", SCHEMA_RELATIONSHIPS)?;

        if relationships.is_empty() {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in relationships {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("Relationships")?;
        }

        log::debug!("[Relationships] Finished writing.");

        Ok(())
    }
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

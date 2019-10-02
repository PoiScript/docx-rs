//! Core File Properties part
//!
//! The corresponding ZIP item is `/docProps/core.xml`.

use docx_codegen::Xml;
use quick_xml::events::BytesStart;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_CORE,
};

#[derive(Debug, Default, Xml)]
#[xml(tag = "cp:coreProperties")]
#[xml(extend_attrs = "core_extend_attrs")]
pub struct Core {
    #[xml(flatten_text = "dc:title")]
    pub title: Option<String>,
    #[xml(flatten_text = "dc:subject")]
    pub subject: Option<String>,
    #[xml(flatten_text = "dc:creator")]
    pub creator: Option<String>,
    #[xml(flatten_text = "cp:keywords")]
    pub keywords: Option<String>,
    #[xml(flatten_text = "dc:description")]
    pub description: Option<String>,
    #[xml(flatten_text = "cp:lastModifiedBy")]
    pub last_modified_by: Option<String>,
    #[xml(flatten_text = "cp:revision")]
    pub revision: Option<String>,
}

#[inline]
fn core_extend_attrs(_: &Core, start: &mut BytesStart) {
    start.push_attribute(("xmlns:cp", SCHEMA_CORE));
}

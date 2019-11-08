//! Core File Properties part
//!
//! The corresponding ZIP item is `/docProps/core.xml`.

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_CORE,
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "cp:coreProperties")]
#[xml(extend_attrs = "core_extend_attrs")]
pub struct Core<'a> {
    #[xml(flatten_text = "dc:title")]
    pub title: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:subject")]
    pub subject: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:creator")]
    pub creator: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:keywords")]
    pub keywords: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:lastModifiedBy")]
    pub last_modified_by: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:revision")]
    pub revision: Option<Cow<'a, str>>,
}

#[inline]
fn core_extend_attrs<W: Write>(_: &Core, mut w: W) -> Result<()> {
    write!(w, " xmlns:cp=\"{}\"", SCHEMA_CORE)?;
    Ok(())
}

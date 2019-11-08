//! Font Table part
//!
//! The corresponding ZIP item is `/word/fontTable.xml`.

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS},
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:fonts")]
#[xml(extend_attrs = "font_table_extend_attrs")]
pub struct FontTable<'a> {
    #[xml(child = "w:font")]
    pub fonts: Vec<Font<'a>>,
}

#[inline]
fn font_table_extend_attrs<W: Write>(_: &FontTable, mut w: W) -> Result<()> {
    write!(&mut w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    write!(&mut w, " xmlns:r=\"{}\"", SCHEMA_RELATIONSHIPS)?;
    Ok(())
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:font")]
pub struct Font<'a> {
    #[xml(attr = "w:name")]
    pub name: Cow<'a, str>,
    #[xml(child = "w:val")]
    pub charset: Option<Charset<'a>>,
    #[xml(child = "w:family")]
    pub family: Option<Family<'a>>,
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:val")]
pub struct Charset<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Charset<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Charset {
            value: value.into(),
        }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:family")]
pub struct Family<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Family<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Family {
            value: value.into(),
        }
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pitch")]
pub struct Pitch<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> Pitch<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        Pitch {
            value: value.into(),
        }
    }
}

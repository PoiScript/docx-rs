//! Font Table part
//!
//! The corresponding ZIP item is `/word/fontTable.xml`.

mod charset;
mod family;
mod font;
mod pitch;

pub use self::{charset::Charset, family::Family, font::Font, pitch::Pitch};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
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

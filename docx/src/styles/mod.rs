//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod style;

pub use self::{default_style::*, style::*};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_MAIN,
};

/// The root element of the styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(child = "w:docDefaults")]
    pub default: Option<DefaultStyle<'a>>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

#[inline]
fn styles_extend_attrs<W: Write>(_: &Styles, mut w: W) -> Result<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
}

impl<'a> Styles<'a> {
    pub fn default(&mut self, style: DefaultStyle<'a>) -> &mut Self {
        self.default = Some(style);
        self
    }

    pub fn push(&mut self, style: Style<'a>) -> &mut Self {
        self.styles.push(style);
        self
    }
}

//! Style Definitions part
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod bold;
mod border;
mod character_style;
mod color;
mod default_style;
mod dstrike;
mod italics;
mod justification;
mod numbers;
mod outline;
mod paragraph_style;
mod size;
mod strike;
mod underline;

pub use self::{
    bold::*, border::*, character_style::*, color::*, default_style::*, dstrike::*, italics::*,
    justification::*, numbers::*, outline::*, paragraph_style::*, size::*, strike::*, underline::*,
};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
    __setter,
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

/// The root element of a style
///
/// This style is applied to a region of a document.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:style")]
#[xml(extend_attrs = "style_extend_attrs")]
pub struct Style<'a> {
    /// Specifies the primary name and the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(child = "w:name")]
    pub name: Option<StyleName<'a>>,
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub para: Option<ParagraphStyle<'a>>,
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub char: Option<CharacterStyle<'a>>,
}

impl<'a> Style<'a> {
    __setter!(name: Option<StyleName<'a>>);
    __setter!(para: Option<ParagraphStyle<'a>>);
    __setter!(char: Option<CharacterStyle<'a>>);
}

#[inline]
fn style_extend_attrs<W: Write>(s: &Style, mut w: W) -> Result<()> {
    write!(w, " w:type=\"paragraph\"")?;
    if let Some(ref name) = s.name {
        write!(w, " w:styleId=\"{}\"", name.value)?;
    }
    Ok(())
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:name")]
pub struct StyleName<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for StyleName<'a> {
    fn from(val: S) -> Self {
        StyleName { value: val.into() }
    }
}

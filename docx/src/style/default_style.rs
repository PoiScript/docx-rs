use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

use super::{character_style::CharacterStyle, paragraph_style::ParagraphStyle};

/// The root element of the default style
///
/// This style is inherited by every paragraph and run.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(child = "w:rPrDefault")]
    pub char: Option<DefaultCharacterStyle<'a>>,
    #[xml(child = "w:pPrDefault")]
    pub para: Option<DefaultParagraphStyle<'a>>,
}

/// The root element of the default character properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharacterStyle<'a> {
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub inner: CharacterStyle<'a>,
}

/// The root element of the default paragraph properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParagraphStyle<'a> {
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub inner: ParagraphStyle<'a>,
}

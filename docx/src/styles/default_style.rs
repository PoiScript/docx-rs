use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    error::{Error, Result},
    formatting::{CharacterProperty, ParagraphProperty},
};

/// Default Style
///
/// This style is inherited by every paragraph and run.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(child = "w:rPrDefault")]
    pub char: Option<DefaultCharacterProperty<'a>>,
    #[xml(child = "w:pPrDefault")]
    pub para: Option<DefaultParagraphProperty<'a>>,
}

impl<'a> DefaultStyle<'a> {
    __setter!(char: Option<DefaultCharacterProperty<'a>>);
    __setter!(para: Option<DefaultParagraphProperty<'a>>);
}

/// Default Character Properties
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharacterProperty<'a> {
    /// character properties
    #[xml(child = "w:rPr")]
    pub inner: CharacterProperty<'a>,
}

impl<'a, T: Into<CharacterProperty<'a>>> From<T> for DefaultCharacterProperty<'a> {
    fn from(val: T) -> Self {
        DefaultCharacterProperty { inner: val.into() }
    }
}

/// Default Paragraph Properties
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParagraphProperty<'a> {
    /// paragraph properties
    #[xml(child = "w:pPr")]
    pub inner: ParagraphProperty<'a>,
}

impl<'a, T: Into<ParagraphProperty<'a>>> From<T> for DefaultParagraphProperty<'a> {
    fn from(val: T) -> Self {
        DefaultParagraphProperty { inner: val.into() }
    }
}

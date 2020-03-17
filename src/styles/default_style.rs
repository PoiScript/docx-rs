use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty},
};

/// Default Style
///
/// This style is inherited by every paragraph and run.
///
/// ```rust
/// use docx::formatting::*;
/// use docx::styles::*;
///
/// let style = DefaultStyle::default()
///     .char(CharacterProperty::default())
///     .para(ParagraphProperty::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:docDefaults")]
pub struct DefaultStyle<'a> {
    #[xml(default, child = "w:rPrDefault")]
    pub char: DefaultCharacterProperty<'a>,
    #[xml(default, child = "w:pPrDefault")]
    pub para: DefaultParagraphProperty<'a>,
}

impl<'a> DefaultStyle<'a> {
    __setter!(char: DefaultCharacterProperty<'a>);
    __setter!(para: DefaultParagraphProperty<'a>);
}

/// Default Character Properties
#[derive(Default, Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharacterProperty<'a> {
    /// character properties
    #[xml(default, child = "w:rPr")]
    pub inner: CharacterProperty<'a>,
}

impl<'a, T: Into<CharacterProperty<'a>>> From<T> for DefaultCharacterProperty<'a> {
    fn from(val: T) -> Self {
        DefaultCharacterProperty { inner: val.into() }
    }
}

/// Default Paragraph Properties
#[derive(Default, Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParagraphProperty<'a> {
    /// paragraph properties
    #[xml(default, child = "w:pPr")]
    pub inner: ParagraphProperty<'a>,
}

impl<'a, T: Into<ParagraphProperty<'a>>> From<T> for DefaultParagraphProperty<'a> {
    fn from(val: T) -> Self {
        DefaultParagraphProperty { inner: val.into() }
    }
}

__xml_test_suites!(
    DefaultStyle,
    DefaultStyle::default(),
    r#"<w:docDefaults><w:rPrDefault><w:rPr/></w:rPrDefault><w:pPrDefault><w:pPr/></w:pPrDefault></w:docDefaults>"#,
);

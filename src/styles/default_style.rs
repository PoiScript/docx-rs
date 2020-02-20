use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter,
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
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rPrDefault")]
pub struct DefaultCharacterProperty<'a> {
    /// character properties
    #[xml(child = "w:rPr")]
    pub inner: Option<CharacterProperty<'a>>,
}

impl<'a, T: Into<Option<CharacterProperty<'a>>>> From<T> for DefaultCharacterProperty<'a> {
    fn from(val: T) -> Self {
        DefaultCharacterProperty { inner: val.into() }
    }
}

/// Default Paragraph Properties
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pPrDefault")]
pub struct DefaultParagraphProperty<'a> {
    /// paragraph properties
    #[xml(child = "w:pPr")]
    pub inner: Option<ParagraphProperty<'a>>,
}

impl<'a, T: Into<Option<ParagraphProperty<'a>>>> From<T> for DefaultParagraphProperty<'a> {
    fn from(val: T) -> Self {
        DefaultParagraphProperty { inner: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        DefaultStyle,
        DefaultStyle::default(),
        r#"<w:docDefaults></w:docDefaults>"#,
        DefaultStyle::default().char(CharacterProperty::default()),
        r#"<w:docDefaults><w:rPrDefault><w:rPr></w:rPr></w:rPrDefault></w:docDefaults>"#,
        DefaultStyle::default().para(ParagraphProperty::default()),
        r#"<w:docDefaults><w:pPrDefault><w:pPr></w:pPr></w:pPrDefault></w:docDefaults>"#,
    );
}

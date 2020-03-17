use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __string_enum, __xml_test_suites,
    formatting::{CharacterProperty, ParagraphProperty},
};

/// Style
///
/// A style that applied to a region of the document.
///
/// ```rust
/// use docx::formatting::*;
/// use docx::styles::*;
///
/// let style = Style::paragraph("style_id")
///     .name("Style Name")
///     .para(ParagraphProperty::default())
///     .char(CharacterProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:style")]
pub struct Style<'a> {
    /// Specifies the type of style.
    #[xml(attr = "w:type")]
    pub ty: StyleType,
    /// Specifies the unique identifier
    ///
    /// This identifier is used throughout the document to apply style in content.
    #[xml(attr = "w:styleId")]
    pub style_id: Cow<'a, str>,
    /// Specifies the primary name
    #[xml(child = "w:name")]
    pub name: Option<StyleName<'a>>,
    /// Specifies a set of paragraph properties
    #[xml(child = "w:pPr")]
    pub para: ParagraphProperty<'a>,
    /// Specifies a set of character properties
    #[xml(child = "w:rPr")]
    pub char: CharacterProperty<'a>,
}

impl<'a> Style<'a> {
    pub fn paragraph<T: Into<Cow<'a, str>>>(style_id: T) -> Self {
        Style {
            ty: StyleType::Paragraph,
            style_id: style_id.into(),
            name: None,
            para: ParagraphProperty::default(),
            char: CharacterProperty::default(),
        }
    }

    pub fn character<T: Into<Cow<'a, str>>>(style_id: T) -> Self {
        Style {
            ty: StyleType::Character,
            style_id: style_id.into(),
            name: None,
            para: ParagraphProperty::default(),
            char: CharacterProperty::default(),
        }
    }

    pub fn table<T: Into<Cow<'a, str>>>(style_id: T) -> Self {
        Style {
            ty: StyleType::Table,
            style_id: style_id.into(),
            name: None,
            para: ParagraphProperty::default(),
            char: CharacterProperty::default(),
        }
    }

    pub fn numbering<T: Into<Cow<'a, str>>>(style_id: T) -> Self {
        Style {
            ty: StyleType::Numbering,
            style_id: style_id.into(),
            name: None,
            para: ParagraphProperty::default(),
            char: CharacterProperty::default(),
        }
    }

    __setter!(ty: StyleType);
    __setter!(name: Option<StyleName<'a>>);
    __setter!(para: ParagraphProperty<'a>);
    __setter!(char: CharacterProperty<'a>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:name")]
pub struct StyleName<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for StyleName<'a> {
    fn from(val: S) -> Self {
        StyleName { value: val.into() }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StyleType {
    Character,
    Paragraph,
    Table,
    Numbering,
}

__string_enum! {
    StyleType {
        Character = "character",
        Paragraph = "paragraph",
        Table = "table",
        Numbering = "numbering",
    }
}

__xml_test_suites!(
    Style,
    Style::numbering("id"),
    r#"<w:style w:type="numbering" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
    Style::table("id").name("name"),
    r#"<w:style w:type="table" w:styleId="id"><w:name w:val="name"/><w:pPr/><w:rPr/></w:style>"#,
    Style::paragraph("id"),
    r#"<w:style w:type="paragraph" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
    Style::character("id"),
    r#"<w:style w:type="character" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
);

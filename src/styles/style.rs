#![allow(unused_must_use)]
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
/// let style = Style::new(StyleType::Paragraph, "style_id")
///     .name("Style Name")
///     .paragraph(ParagraphProperty::default())
///     .character(CharacterProperty::default());
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
    #[xml(default, child = "w:pPr")]
    pub paragraph: ParagraphProperty<'a>,
    /// Specifies a set of character properties
    #[xml(default, child = "w:rPr")]
    pub character: CharacterProperty<'a>,
}

impl<'a> Style<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(ty: StyleType, style_id: T) -> Self {
        Style {
            ty,
            style_id: style_id.into(),
            name: None,
            paragraph: ParagraphProperty::default(),
            character: CharacterProperty::default(),
        }
    }

    __setter!(ty: StyleType);
    __setter!(name: Option<StyleName<'a>>);
    __setter!(paragraph: ParagraphProperty<'a>);
    __setter!(character: CharacterProperty<'a>);
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
    Style::new(StyleType::Numbering, "id"),
    r#"<w:style w:type="numbering" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
    Style::new(StyleType::Table, "id").name("name"),
    r#"<w:style w:type="table" w:styleId="id"><w:name w:val="name"/><w:pPr/><w:rPr/></w:style>"#,
    Style::new(StyleType::Paragraph, "id"),
    r#"<w:style w:type="paragraph" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
    Style::new(StyleType::Character, "id"),
    r#"<w:style w:type="character" w:styleId="id"><w:pPr/><w:rPr/></w:style>"#,
);

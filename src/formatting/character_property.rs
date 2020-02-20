use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter,
    formatting::{Bold, Color, Dstrike, Italics, Outline, Size, Strike, Underline},
};

/// Character Property
///
/// ```rust
/// use docx::formatting::{CharacterProperty, UnderlineStyle};
///
/// let prop = CharacterProperty::default()
///     .style_id("foo")
///     .color("00ff00")
///     .color(0xff0000)
///     .color((0x00, 0x00, 0xff))
///     .size(42usize)
///     .bold(true)
///     .italics(false)
///     .strike(true)
///     .dstrike(false)
///     .outline(true)
///     .underline("00ff00")
///     .underline(("ff0000", UnderlineStyle::Dash));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:rPr")]
pub struct CharacterProperty<'a> {
    /// Specifies the style ID of the character style.
    #[xml(child = "w:rStyle")]
    pub style_id: Option<CharacterStyleId<'a>>,
    /// Specifies the color to be used to display text.
    #[xml(child = "w:color")]
    pub color: Option<Color<'a>>,
    /// Specifies the font size in half points.
    #[xml(child = "w:sz")]
    pub size: Option<Size>,
    /// Specifies that the text of the text run is to be bold.
    #[xml(child = "w:b")]
    pub bold: Option<Bold>,
    /// Specifies that the text of the text run is to be italics.
    #[xml(child = "w:i")]
    pub italics: Option<Italics>,
    /// Specifies that the contents are to be displayed with a horizontal line through the center of the line.
    #[xml(child = "w:strike")]
    pub strike: Option<Strike>,
    /// Specifies that the contents are to be displayed with two horizontal lines through each character.
    #[xml(child = "w:dstrike")]
    pub dstrike: Option<Dstrike>,
    /// Specifies that the content should be displayed as if it had an outline.
    #[xml(child = "w:outline")]
    pub outline: Option<Outline>,
    /// Specifies that the content should be displayed with an underline
    #[xml(child = "w:u")]
    pub underline: Option<Underline<'a>>,
}

impl<'a> CharacterProperty<'a> {
    __setter!(style_id: Option<CharacterStyleId<'a>>);
    __setter!(color: Option<Color<'a>>);
    __setter!(bold: Option<Bold>);
    __setter!(dstrike: Option<Dstrike>);
    __setter!(italics: Option<Italics>);
    __setter!(outline: Option<Outline>);
    __setter!(strike: Option<Strike>);
    __setter!(size: Option<Size>);
    __setter!(underline: Option<Underline<'a>>);
}

#[derive(Debug, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:rStyle")]
pub struct CharacterStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for CharacterStyleId<'a> {
    fn from(val: T) -> Self {
        CharacterStyleId { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        CharacterProperty,
        CharacterProperty::default(),
        r#"<w:rPr></w:rPr>"#,
        CharacterProperty::default().style_id(""),
        r#"<w:rPr><w:rStyle w:val=""/></w:rPr>"#,
        CharacterProperty::default().color("00ff00"),
        r#"<w:rPr><w:color w:val="00ff00"/></w:rPr>"#,
        CharacterProperty::default().size(42usize),
        r#"<w:rPr><w:sz w:val="42"/></w:rPr>"#,
        CharacterProperty::default().bold(true),
        r#"<w:rPr><w:b w:val="true"/></w:rPr>"#,
        CharacterProperty::default().italics(false),
        r#"<w:rPr><w:i w:val="false"/></w:rPr>"#,
        CharacterProperty::default().outline(true),
        r#"<w:rPr><w:outline w:val="true"/></w:rPr>"#,
        CharacterProperty::default().strike(false),
        r#"<w:rPr><w:strike w:val="false"/></w:rPr>"#,
        CharacterProperty::default().dstrike(true),
        r#"<w:rPr><w:dstrike w:val="true"/></w:rPr>"#,
        CharacterProperty::default().underline(Underline::default()),
        r#"<w:rPr><w:u/></w:rPr>"#,
    );
}

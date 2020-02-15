use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    error::{Error, Result},
};

use super::{
    bold::Bold, color::Color, dstrike::Dstrike, italics::Italics, outline::Outline, size::Size,
    strike::Strike, underline::Underline,
};

/// The root element of a set of character properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:rPr")]
pub struct CharacterStyle<'a> {
    /// Specifies the color to be used to display text.
    #[xml(child = "w:color")]
    pub color: Option<Color<'a>>,
    /// Specifies the font size in half points.
    #[xml(child = "w:sz")]
    pub sz: Option<Size>,
    /// Specifies that the text of the text run is to be bold.
    #[xml(child = "w:b")]
    pub bold: Option<Bold>,
    /// Specifies that the text of the text run is to be italics.
    #[xml(child = "w:i")]
    pub italics: Option<Italics>,
    /// Specifies that the contents are to be displayed with a horizontal line through the center of the line.
    #[xml(child = "w:strike")]
    pub strike: Option<Strike>,
    #[xml(child = "w:dstrike")]
    pub dstrike: Option<Dstrike>,
    #[xml(child = "w:outline")]
    pub outline: Option<Outline>,
    #[xml(child = "w:u")]
    pub underline: Option<Underline<'a>>,
}

impl<'a> CharacterStyle<'a> {
    __setter!(color: Option<Color<'a>>);
    __setter!(bold: Option<Bold>);
    __setter!(dstrike: Option<Dstrike>);
    __setter!(italics: Option<Italics>);
    __setter!(outline: Option<Outline>);
    __setter!(strike: Option<Strike>);
    __setter!(sz: Option<Size>);
    __setter!(underline: Option<Underline<'a>>);
}

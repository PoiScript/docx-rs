use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

use super::{
    bold::Bold,
    color::Color,
    dstrike::Dstrike,
    italics::Italics,
    outline::Outline,
    size::Size,
    strike::Strike,
    underline::{Underline, UnderlineStyle},
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
    pub fn sz(&mut self, sz: usize) -> &mut Self {
        self.sz = Some(Size::new(sz));
        self
    }

    pub fn reset_sz(&mut self) -> &mut Self {
        self.sz = None;
        self
    }

    pub fn color(&mut self, color: &str) -> &mut Self {
        self.color = Some(Color::new(color.to_owned()));
        self
    }

    pub fn reset_color(&mut self) -> &mut Self {
        self.color = None;
        self
    }

    pub fn bold(&mut self, val: bool) -> &mut Self {
        self.bold = Some(Bold::new(val));
        self
    }

    pub fn reset_bold(&mut self) -> &mut Self {
        self.bold = None;
        self
    }

    pub fn italics(&mut self, val: bool) -> &mut Self {
        self.italics = Some(Italics::new(val));
        self
    }

    pub fn reset_italics(&mut self) -> &mut Self {
        self.italics = None;
        self
    }

    pub fn strike(&mut self, val: bool) -> &mut Self {
        self.strike = Some(Strike::new(val));
        if let Some(Dstrike { value: true }) = self.dstrike {
            self.dstrike = None;
        }
        self
    }

    pub fn reset_strike(&mut self) -> &mut Self {
        self.strike = None;
        self
    }

    pub fn dstrike(&mut self, val: bool) -> &mut Self {
        self.dstrike = Some(Dstrike::new(val));
        if let Some(Strike { value: true }) = self.strike {
            self.strike = None;
        }
        self
    }

    pub fn reset_dstrike(&mut self) -> &mut Self {
        self.dstrike = None;
        self
    }

    pub fn outline(&mut self, val: bool) -> &mut Self {
        self.outline = Some(Outline::new(val));
        self
    }

    pub fn reset_outline(&mut self) -> &mut Self {
        self.outline = None;
        self
    }

    pub fn underline(&mut self, color: Option<&'a str>, ty: Option<UnderlineStyle>) -> &mut Self {
        self.underline = Some(Underline {
            color: color.map(Into::into),
            val: ty,
        });
        self
    }

    pub fn reset_underline(&mut self) -> &mut Self {
        self.underline = None;
        self
    }
}

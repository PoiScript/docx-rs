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
mod jc;
mod numbers;
mod outline;
mod paragraph_style;
mod size;
mod strike;
mod underline;

pub use self::{
    border::{
        BetweenBorder, BorderStyle, Borders, BottomBorder, LeftBorder, RightBorder, TopBorder,
    },
    character_style::CharacterStyle,
    default_style::DefaultStyle,
    jc::Justification,
    paragraph_style::ParagraphStyle,
};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;
use std::io::Write;

use crate::{
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
    /// Appends a style to the back of the styles, and returns it.
    pub fn create_style(&mut self) -> &mut Style<'a> {
        self.styles.push(Style::default());
        self.styles.last_mut().unwrap()
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

impl<'a> StyleName<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        StyleName {
            value: value.into(),
        }
    }
}

impl<'a> Style<'a> {
    /// Setting the name of this style
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(StyleName::new(name.to_owned()));
        self
    }

    /// Resetting the name of this style
    pub fn reset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }

    /// Returns the paragraph properties
    pub fn para(&mut self) -> &mut ParagraphStyle<'a> {
        self.para.get_or_insert(ParagraphStyle::default())
    }

    /// Returns the character properties
    pub fn char(&mut self) -> &mut CharacterStyle<'a> {
        self.char.get_or_insert(CharacterStyle::default())
    }
}

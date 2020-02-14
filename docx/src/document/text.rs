use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// The root element of a literal text that shall be displayed in the document
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:t")]
pub struct Text<'a> {
    /// Specifies how to handle whitespace
    #[xml(attr = "xml:space")]
    pub space: Option<TextSpace>,
    /// Specifies a literal text
    #[xml(text)]
    pub text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(text: S, space: Option<TextSpace>) -> Self {
        Text {
            text: text.into(),
            space,
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(text: &'a str) -> Self {
        Text {
            space: None,
            text: text.into(),
        }
    }
}

/// Specifies how whitespace should be handled
#[derive(Debug)]
pub enum TextSpace {
    Default,
    /// Using the W3C space preservation rules
    Preserve,
}

impl Default for TextSpace {
    fn default() -> Self {
        TextSpace::Default
    }
}

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

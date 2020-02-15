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

impl From<String> for Text<'_> {
    fn from(val: String) -> Self {
        Text {
            text: val.into(),
            space: None,
        }
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(val: &'a str) -> Self {
        Text {
            text: val.into(),
            space: None,
        }
    }
}

impl From<(String, TextSpace)> for Text<'_> {
    fn from(val: (String, TextSpace)) -> Self {
        Text {
            text: val.0.into(),
            space: Some(val.1),
        }
    }
}

impl<'a> From<(&'a str, TextSpace)> for Text<'a> {
    fn from(val: (&'a str, TextSpace)) -> Self {
        Text {
            text: val.0.into(),
            space: Some(val.1),
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

__string_enum! {
    TextSpace {
        Default = "default",
        Preserve = "preserve",
    }
}

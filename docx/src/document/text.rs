use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// Literal Text
///
/// A literal text that shall be displayed in the document.
///
/// ```rust
/// use docx::document::{Text, TextSpace};
///
/// let _: Text = "text".into();
/// let _: Text = String::from("text").into();
/// let _: Text = ("text", TextSpace::Preserve).into();
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
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

/// Text Space Rules
///
/// Specifies how whitespace should be handled in a literal text.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TextSpace {
    /// Default rules
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Text {
                text: Cow::Borrowed("text"),
                space: None,
            },
            "text".into(),
        );
        assert_eq!(
            Text {
                text: Cow::Borrowed("text"),
                space: None,
            },
            String::from("text").into(),
        );
        assert_eq!(
            Text {
                text: Cow::Borrowed("text"),
                space: Some(TextSpace::Preserve)
            },
            ("text", TextSpace::Preserve).into(),
        );
        assert_eq!(
            Text {
                text: Cow::Borrowed("text"),
                space: Some(TextSpace::Default)
            },
            (String::from("text"), TextSpace::Default).into(),
        );
    }
}

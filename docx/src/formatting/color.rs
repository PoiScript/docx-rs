use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

/// Text Color
///
/// Specifies the color to be used to display text.
///
/// ```rust
/// use docx::formatting::Color;
///
/// let _: Color = "000000".into();
/// let _: Color = String::from("000000").into();
/// let _: Color = 0u32.into(); // "000000"
/// let _: Color = (0u8, 0u8, 0u8).into(); // "000000"
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:color")]
pub struct Color<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> From<&'a str> for Color<'a> {
    fn from(val: &'a str) -> Self {
        Color {
            value: Cow::Borrowed(val),
        }
    }
}

impl From<String> for Color<'_> {
    fn from(val: String) -> Self {
        Color {
            value: Cow::Owned(val),
        }
    }
}

impl From<u32> for Color<'_> {
    fn from(val: u32) -> Self {
        Color {
            value: Cow::Owned(format!("{:06x}", val)),
        }
    }
}

impl From<(u8, u8, u8)> for Color<'_> {
    fn from(val: (u8, u8, u8)) -> Self {
        Color {
            value: Cow::Owned(format!("{:02x}{:02x}{:02x}", val.0, val.1, val.2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Color {
                value: Cow::Borrowed("000000")
            },
            "000000".into(),
        );
        assert_eq!(
            Color {
                value: Cow::Borrowed("000000")
            },
            String::from("000000").into(),
        );
        assert_eq!(
            Color {
                value: Cow::Borrowed("000000")
            },
            0u32.into(),
        );
        assert_eq!(
            Color {
                value: Cow::Borrowed("000000")
            },
            (0u8, 0u8, 0u8).into(),
        );
    }
}

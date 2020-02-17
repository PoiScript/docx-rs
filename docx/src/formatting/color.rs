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
/// let color = Color::from("000000");
/// let color = Color::from(String::from("000000"));
/// let color = Color::from(0u32); // "000000"
/// let color = Color::from((0u8, 0u8, 0u8)); // "000000"
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
    use crate::__test_read_write;

    __test_read_write!(
        Color,
        Color::from("000000"),
        r#"<w:color w:val="000000"/>"#,
        Color::from(0u32),
        r#"<w:color w:val="000000"/>"#,
        Color::from((0u8, 0u8, 0u8)),
        r#"<w:color w:val="000000"/>"#,
    );
}

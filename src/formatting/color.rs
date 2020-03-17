use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

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
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:color")]
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

__xml_test_suites!(
    Color,
    Color::from("000000"),
    r#"<w:color w:val="000000"/>"#,
    Color::from(0u32),
    r#"<w:color w:val="000000"/>"#,
    Color::from((0u8, 0u8, 0u8)),
    r#"<w:color w:val="000000"/>"#,
);

use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Indent Level
///
/// ```rust
/// use docx::formatting::*;
///
/// let lvl = IndentLevel::from(42usize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:ilvl")]
pub struct IndentLevel {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for IndentLevel {
    fn from(val: T) -> Self {
        IndentLevel { value: val.into() }
    }
}

__xml_test_suites!(
    IndentLevel,
    IndentLevel::from(40usize),
    r#"<w:ilvl w:val="40"/>"#,
);

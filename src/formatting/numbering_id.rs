use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Numbering Id
///
/// ```rust
/// use docx::formatting::*;
///
/// let id = NumberingId::from(42usize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:numId")]
pub struct NumberingId {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for NumberingId {
    fn from(val: T) -> Self {
        NumberingId { value: val.into() }
    }
}

__xml_test_suites!(
    NumberingId,
    NumberingId::from(40usize),
    r#"<w:numId w:val="40"/>"#,
);

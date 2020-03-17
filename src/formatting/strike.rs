use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Strike
///
/// ```rust
/// use docx::formatting::*;
///
/// let strike = Strike::from(false);
/// let strike = Strike::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:strike")]
pub struct Strike {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Strike {
    fn from(val: T) -> Self {
        Strike { value: val.into() }
    }
}

__xml_test_suites!(
    Strike,
    Strike::default(),
    r#"<w:strike/>"#,
    Strike::from(false),
    r#"<w:strike w:val="false"/>"#,
    Strike::from(true),
    r#"<w:strike w:val="true"/>"#,
);

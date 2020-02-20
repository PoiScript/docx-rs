use strong_xml::{XmlRead, XmlWrite};

/// Italics
///
/// ```rust
/// use docx::formatting::*;
///
/// let i = Italics::from(false);
/// let i = Italics::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:i")]
pub struct Italics {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Italics {
    fn from(val: T) -> Self {
        Italics { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Italics,
        Italics::default(),
        r#"<w:i/>"#,
        Italics::from(false),
        r#"<w:i w:val="false"/>"#,
        Italics::from(true),
        r#"<w:i w:val="true"/>"#,
    );
}

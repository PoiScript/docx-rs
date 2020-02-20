use strong_xml::{XmlRead, XmlWrite};

/// Bold
///
/// ```rust
/// use docx::formatting::*;
///
/// let bold = Bold::from(false);
/// let bold = Bold::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:b")]
pub struct Bold {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Bold {
    fn from(val: T) -> Self {
        Bold { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Bold,
        Bold::default(),
        r#"<w:b/>"#,
        Bold::from(false),
        r#"<w:b w:val="false"/>"#,
        Bold::from(true),
        r#"<w:b w:val="true"/>"#,
    );
}

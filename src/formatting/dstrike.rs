use strong_xml::{XmlRead, XmlWrite};

/// Double Strike
///
/// ```rust
/// use docx::formatting::*;
///
/// let dstrike = Dstrike::from(false);
/// let dstrike = Dstrike::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:dstrike")]
pub struct Dstrike {
    #[xml(attr = "w:val")]
    pub value: Option<bool>,
}

impl<T: Into<Option<bool>>> From<T> for Dstrike {
    fn from(val: T) -> Self {
        Dstrike { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Dstrike,
        Dstrike::default(),
        r#"<w:dstrike/>"#,
        Dstrike::from(false),
        r#"<w:dstrike w:val="false"/>"#,
        Dstrike::from(true),
        r#"<w:dstrike w:val="true"/>"#,
    );
}

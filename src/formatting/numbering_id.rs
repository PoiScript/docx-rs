use strong_xml::{XmlRead, XmlWrite};

/// Numbering Id
///
/// ```rust
/// use docx::formatting::*;
///
/// let id = NumberingId::from(42usize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:numId")]
pub struct NumberingId {
    #[xml(attr = "w:val")]
    pub value: usize,
}

impl<T: Into<usize>> From<T> for NumberingId {
    fn from(val: T) -> Self {
        NumberingId { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        NumberingId,
        NumberingId::from(40usize),
        r#"<w:numId w:val="40"/>"#,
    );
}

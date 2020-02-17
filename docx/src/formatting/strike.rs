use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

/// Strike
///
/// ```rust
/// use docx::formatting::*;
///
/// let strike = Strike::from(false);
/// let strike = Strike::from(true);
/// ```
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:strike")]
pub struct Strike {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Strike {
    fn from(val: T) -> Self {
        Strike { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Strike,
        Strike::from(false),
        r#"<w:strike w:val="false"/>"#,
        Strike::from(true),
        r#"<w:strike w:val="true"/>"#,
    );
}

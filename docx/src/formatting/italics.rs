use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

/// Italics
///
/// ```rust
/// use docx::formatting::*;
///
/// let i = Italics::from(false);
/// let i = Italics::from(true);
/// ```
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:i")]
pub struct Italics {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Italics {
    fn from(value: T) -> Self {
        Italics {
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Italics,
        Italics::from(false),
        r#"<w:i w:val="false"/>"#,
        Italics::from(true),
        r#"<w:i w:val="true"/>"#,
    );
}

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

/// Outline
///
/// ```rust
/// use docx::formatting::*;
///
/// let outline = Outline::from(false);
/// let outline = Outline::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:outline")]
pub struct Outline {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl<T: Into<bool>> From<T> for Outline {
    fn from(val: T) -> Self {
        Outline { value: val.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Outline,
        Outline::from(false),
        r#"<w:outline w:val="false"/>"#,
        Outline::from(true),
        r#"<w:outline w:val="true"/>"#,
    );
}

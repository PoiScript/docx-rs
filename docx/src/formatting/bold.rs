use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

/// Bold
///
/// ```rust
/// use docx::formatting::*;
///
/// let bold = Bold::from(false);
/// let bold = Bold::from(true);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:b")]
pub struct Bold {
    #[xml(attr = "w:val")]
    pub value: bool,
}

impl From<bool> for Bold {
    fn from(value: bool) -> Self {
        Bold { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Bold,
        Bold::from(false),
        r#"<w:b w:val="false"/>"#,
        Bold::from(true),
        r#"<w:b w:val="true"/>"#,
    );
}

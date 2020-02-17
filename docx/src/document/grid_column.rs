use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

/// Grid Column
///
/// ```rust
/// use docx::document::*;
///
/// let col = GridColumn::from(42);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:gridCol")]
pub struct GridColumn {
    #[xml(attr = "w:w")]
    pub width: usize,
}

impl From<usize> for GridColumn {
    fn from(width: usize) -> GridColumn {
        GridColumn { width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        GridColumn,
        GridColumn::from(42usize),
        r#"<w:gridCol w:w="42"></w:gridCol>"#,
    );
}

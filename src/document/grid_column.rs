use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;

/// Grid Column
///
/// ```rust
/// use docx::document::*;
///
/// let col = GridColumn::from(42);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
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

__xml_test_suites!(
    GridColumn,
    GridColumn::from(42usize),
    r#"<w:gridCol w:w="42"/>"#,
);

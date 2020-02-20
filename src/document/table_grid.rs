use strong_xml::{XmlRead, XmlWrite};

use crate::document::GridColumn;

/// Table Grid
///
/// ```rust
/// use docx::document::*;
///
/// let grid = TableGrid::from(vec![42, 42]);
///
/// let grid = TableGrid::default()
///     .push_column(42)
///     .push_column(42);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblGrid")]
pub struct TableGrid {
    #[xml(child = "w:gridCol")]
    pub columns: Vec<GridColumn>,
}

impl TableGrid {
    pub fn push_column<T: Into<GridColumn>>(mut self, col: T) -> Self {
        self.columns.push(col.into());
        self
    }
}

impl From<Vec<usize>> for TableGrid {
    fn from(cols: Vec<usize>) -> TableGrid {
        TableGrid {
            columns: cols.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TableGrid,
        TableGrid::default(),
        "<w:tblGrid></w:tblGrid>",
        TableGrid::default().push_column(42),
        r#"<w:tblGrid><w:gridCol w:w="42"></w:gridCol></w:tblGrid>"#,
        TableGrid::default().push_column(42).push_column(42),
        r#"<w:tblGrid><w:gridCol w:w="42"></w:gridCol><w:gridCol w:w="42"></w:gridCol></w:tblGrid>"#,
        TableGrid::from(vec![42, 42]),
        r#"<w:tblGrid><w:gridCol w:w="42"></w:gridCol><w:gridCol w:w="42"></w:gridCol></w:tblGrid>"#,
    );
}

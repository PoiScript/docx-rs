use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    document::{TableGrid, TableRow},
    error::{Error, Result},
    formatting::TableProperty,
};

/// Table
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let tbl = Table::default()
///     .prop(TableProperty::default())
///     .push_grid(vec![1, 2, 3])
///     .push_grid(TableGrid::default())
///     .push_row(TableRow::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tbl")]
pub struct Table<'a> {
    #[xml(child = "w:tblPr")]
    pub prop: Option<TableProperty<'a>>,
    #[xml(child = "w:tblGrid")]
    pub grids: Vec<TableGrid>,
    #[xml(child = "w:tr")]
    pub rows: Vec<TableRow<'a>>,
}

impl<'a> Table<'a> {
    __setter!(prop: Option<TableProperty<'a>>);

    pub fn push_grid<T: Into<TableGrid>>(mut self, grid: T) -> Self {
        self.grids.push(grid.into());
        self
    }

    pub fn push_row<T: Into<TableRow<'a>>>(mut self, row: T) -> Self {
        self.rows.push(row.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Table,
        Table::default(),
        "<w:tbl></w:tbl>",
        Table::default().prop(TableProperty::default()),
        "<w:tbl><w:tblPr></w:tblPr></w:tbl>",
        Table::default().push_grid(TableGrid::default()),
        "<w:tbl><w:tblGrid></w:tblGrid></w:tbl>",
        Table::default().push_row(TableRow::default()),
        "<w:tbl><w:tr></w:tr></w:tbl>",
    );
}

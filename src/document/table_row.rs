use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, document::TableCell, formatting::TableRowProperty};

/// Table Row
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let row = TableRow::default()
///     .prop(TableRowProperty::default())
///     .push_cell(Paragraph::default())
///     .push_cell(
///         TableCell::pargraph(Paragraph::default())
///             .prop(TableCellProperty::default())
///     );
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tr")]
pub struct TableRow<'a> {
    #[xml(child = "w:trPr")]
    pub prop: Option<TableRowProperty>,
    #[xml(child = "w:tc")]
    pub cells: Vec<TableCell<'a>>,
}

impl<'a> TableRow<'a> {
    __setter!(prop: Option<TableRowProperty>);

    pub fn push_cell<T: Into<TableCell<'a>>>(mut self, cell: T) -> Self {
        self.cells.push(cell.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;
    use crate::document::Paragraph;

    __test_read_write!(
        TableRow,
        TableRow::default(),
        "<w:tr></w:tr>",
        TableRow::default().prop(TableRowProperty::default()),
        "<w:tr><w:trPr></w:trPr></w:tr>",
        TableRow::default().push_cell(Paragraph::default()),
        "<w:tr><w:tc><w:p></w:p></w:tc></w:tr>",
    );
}

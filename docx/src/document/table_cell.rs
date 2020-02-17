use derive_more::From;
use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    document::Paragraph,
    error::{Error, Result},
    formatting::TableCellProperty,
};

/// Table Cell
///
/// ```rust
/// use docx::document::*;
/// use docx::formatting::*;
///
/// let cell = TableCell::from(Paragraph::default());
///
/// let cell = TableCell::pargraph(Paragraph::default())
///     .prop(TableCellProperty::default());
/// ```
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tc")]
pub struct TableCell<'a> {
    #[xml(child = "w:tcPr")]
    pub prop: Option<TableCellProperty>,
    #[xml(child = "w:p")]
    pub content: TableCellContent<'a>,
}

impl<'a> TableCell<'a> {
    __setter!(prop: Option<TableCellProperty>);

    pub fn pargraph<T: Into<Paragraph<'a>>>(par: T) -> Self {
        TableCell {
            prop: None,
            content: TableCellContent::Paragraph(par.into()),
        }
    }
}

impl<'a, T: Into<TableCellContent<'a>>> From<T> for TableCell<'a> {
    fn from(content: T) -> Self {
        TableCell {
            prop: None,
            content: content.into(),
        }
    }
}

#[derive(Debug, From, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableCellContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    // #[xml(tag = "w:tbl")]
    // Table(Table<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;
    use crate::document::Paragraph;

    __test_read_write!(
        TableCell,
        TableCell::pargraph(Paragraph::default()),
        "<w:tc><w:p></w:p></w:tc>",
        TableCell::pargraph(Paragraph::default()).prop(TableCellProperty::default()),
        "<w:tc><w:tcPr></w:tcPr><w:p></w:p></w:tc>",
    );
}

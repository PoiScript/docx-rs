use derive_more::From;
use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __setter,
    document::{Paragraph, Table},
    error::{Error, Result},
    formatting::TableCellProperty,
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
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

#[derive(Debug, From, XmlRead, XmlWrite, IntoOwned)]
pub enum TableCellContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    // #[xml(tag = "w:tbl")]
    // Table(Table<'a>),
}

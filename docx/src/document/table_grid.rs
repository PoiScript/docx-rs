use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    document::GridColumn,
    error::{Error, Result},
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:tblGrid")]
pub struct TableGrid {
    #[xml(child = "w:gridCol")]
    pub columns: Vec<GridColumn>,
}

impl From<Vec<usize>> for TableGrid {
    fn from(cols: Vec<usize>) -> TableGrid {
        TableGrid {
            columns: cols.into_iter().map(Into::into).collect(),
        }
    }
}

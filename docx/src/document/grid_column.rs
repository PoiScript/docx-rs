use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::error::{Error, Result};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
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

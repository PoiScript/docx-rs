use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// The root element of a break
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:br")]
pub struct Break {
    /// Specifies the break type of this break.
    #[xml(attr = "type")]
    pub ty: Option<BreakType>,
}

impl<T: Into<Option<BreakType>>> From<T> for Break {
    fn from(val: T) -> Self {
        Break { ty: val.into() }
    }
}

/// Specifies the break type of a break
///
/// The default value is TextWrapping.
#[derive(Debug)]
pub enum BreakType {
    /// Text restarts on the next column.
    Column,
    /// Text restarts on the next page.
    Page,
    /// Text restarts on the next line.
    TextWrapping,
}

__string_enum! {
    BreakType {
        Column = "column",
        Page = "page",
        TextWrapping = "textWrapping",
    }
}

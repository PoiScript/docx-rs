use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:jc")]
pub struct Justification {
    #[xml(attr = "w:val")]
    pub value: JustificationVal,
}

impl From<JustificationVal> for Justification {
    fn from(value: JustificationVal) -> Self {
        Justification { value }
    }
}

#[derive(Debug)]
pub enum JustificationVal {
    Start,
    End,
    Center,
    Both,
    Distribute,
    Right,
    Left,
}

__string_enum! {
    JustificationVal {
        Start = "start",
        End = "end",
        Center = "center",
        Both = "both",
        Distribute = "distribute",
        Right = "right",
        Left = "left",
    }
}

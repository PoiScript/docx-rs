use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:jc")]
pub struct Jc {
    #[xml(attr = "w:val")]
    pub value: Justification,
}

impl Jc {
    pub fn new(value: Justification) -> Self {
        Jc { value }
    }
}

#[derive(Debug)]
pub enum Justification {
    Start,
    End,
    Center,
    Both,
    Distribute,
    Right,
    Left,
}

__string_enum! {
    Justification {
        Start = "start",
        End = "end",
        Center = "center",
        Both = "both",
        Distribute = "distribute",
        Right = "right",
        Left = "left",
    }
}

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// Justification
///
/// ```rust
/// use docx::formatting::*;
///
/// let jc = Justification::from(JustificationVal::Start);
/// ```
#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
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
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Justification,
        Justification::from(JustificationVal::Start),
        r#"<w:jc w:val="start"/>"#,
    );
}

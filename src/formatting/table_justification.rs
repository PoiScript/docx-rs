use strong_xml::{XmlRead, XmlWrite};

use crate::__string_enum;

/// Table Justification
///
/// ```rust
/// use docx::formatting::*;
///
/// let jc = TableJustification::from(TableJustificationVal::Start);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:jc")]
pub struct TableJustification {
    #[xml(attr = "w:val")]
    pub value: Option<TableJustificationVal>,
}

impl From<TableJustificationVal> for TableJustification {
    fn from(val: TableJustificationVal) -> Self {
        TableJustification { value: Some(val) }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TableJustificationVal {
    Start,
    End,
    Center,
}

__string_enum! {
    TableJustificationVal {
        Start = "start",
        End = "end",
        Center = "center",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TableJustification,
        TableJustification::default(),
        "<w:jc/>",
        TableJustification::from(TableJustificationVal::Start),
        r#"<w:jc w:val="start"/>"#,
    );
}

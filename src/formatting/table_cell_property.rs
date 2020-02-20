use strong_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperty {}

impl TableCellProperty {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TableCellProperty,
        TableCellProperty::default(),
        r#"<w:tcPr></w:tcPr>"#,
    );
}

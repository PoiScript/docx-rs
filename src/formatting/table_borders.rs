use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter,
    formatting::{BottomBorder, TopBorder},
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        TableBorders,
        TableBorders::default(),
        r#"<w:tblBorders></w:tblBorders>"#,
        TableBorders::default().top(TopBorder::default()),
        r#"<w:tblBorders><w:top/></w:tblBorders>"#,
        TableBorders::default().bottom(BottomBorder::default()),
        r#"<w:tblBorders><w:bottom/></w:tblBorders>"#,
    );
}

use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{BottomBorder, TopBorder, LeftBorder, RightBorder, InsideHBorder, InsideVBorder},
};

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "w:insideH")]
    pub insideH: Option<InsideHBorder<'a>>,
    #[xml(child = "w:insideV")]
    pub insideV: Option<InsideVBorder<'a>>,
}

impl<'a> TableBorders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
    __setter!(left: Option<LeftBorder<'a>>);
    __setter!(right: Option<RightBorder<'a>>);
    __setter!(insideH: Option<InsideHBorder<'a>>);
    __setter!(insideV: Option<InsideVBorder<'a>>);
}

__xml_test_suites!(
    TableBorders,
    TableBorders::default(),
    r#"<w:tblBorders/>"#,
    TableBorders::default().top(TopBorder::default()),
    r#"<w:tblBorders><w:top/></w:tblBorders>"#,
    TableBorders::default().bottom(BottomBorder::default()),
    r#"<w:tblBorders><w:bottom/></w:tblBorders>"#,
    TableBorders::default().left(LeftBorder::default()),
    r#"<w:tblBorders><w:left/></w:tblBorders>"#,
    TableBorders::default().right(RightBorder::default()),
    r#"<w:tblBorders><w:right/></w:tblBorders>"#,
    TableBorders::default().insideH(InsideHBorder::default()),
    r#"<w:tblBorders><w:right/></w:tblBorders>"#,
    TableBorders::default().insideV(InsideVBorder::default()),
    r#"<w:tblBorders><w:right/></w:tblBorders>"#,
);

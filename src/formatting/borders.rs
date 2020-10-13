use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{BetweenBorder, BottomBorder, LeftBorder, RightBorder, TopBorder, InsideHBorder, InsideVBorder},
};

/// Borders
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pBdr")]
pub struct Borders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub bottom: Option<BottomBorder<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "w:between")]
    pub between: Option<BetweenBorder<'a>>,
    #[xml(child = "w:insideH")]
    pub insideH: Option<InsideHBorder<'a>>,
    #[xml(child = "w:insideV")]
    pub insideV: Option<InsideVBorder<'a>>,
}

impl<'a> Borders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
    __setter!(left: Option<LeftBorder<'a>>);
    __setter!(right: Option<RightBorder<'a>>);
    __setter!(between: Option<BetweenBorder<'a>>);
    __setter!(insideH: Option<InsideHBorder<'a>>);
    __setter!(insideV: Option<InsideVBorder<'a>>);
}

__xml_test_suites!(
    Borders,
    Borders::default(),
    r#"<w:pBdr/>"#,
    Borders::default().top(TopBorder::default()),
    r#"<w:pBdr><w:top/></w:pBdr>"#,
    Borders::default().bottom(BottomBorder::default()),
    r#"<w:pBdr><w:bottom/></w:pBdr>"#,
    Borders::default().left(LeftBorder::default()),
    r#"<w:pBdr><w:left/></w:pBdr>"#,
    Borders::default().right(RightBorder::default()),
    r#"<w:pBdr><w:right/></w:pBdr>"#,
    Borders::default().between(BetweenBorder::default()),
    r#"<w:pBdr><w:between/></w:pBdr>"#,
    Borders::default().insideH(InsideHBorder::default()),
    r#"<w:pBdr><w:insideH/></w:pBdr>"#,
    Borders::default().insideV(InsideVBorder::default()),
    r#"<w:pBdr><w:insideV/></w:pBdr>"#,
);

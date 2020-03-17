use strong_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites,
    formatting::{BetweenBorder, BottomBorder, LeftBorder, RightBorder, TopBorder},
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
}

impl<'a> Borders<'a> {
    __setter!(top: Option<TopBorder<'a>>);
    __setter!(bottom: Option<BottomBorder<'a>>);
    __setter!(left: Option<LeftBorder<'a>>);
    __setter!(right: Option<RightBorder<'a>>);
    __setter!(between: Option<BetweenBorder<'a>>);
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
);

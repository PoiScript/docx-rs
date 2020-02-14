use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct Borders<'a> {
    #[xml(child = "w:top")]
    pub top: Option<TopBorder<'a>>,
    #[xml(child = "w:bottom")]
    pub botton: Option<BottomBorder<'a>>,
    #[xml(child = "w:left")]
    pub left: Option<LeftBorder<'a>>,
    #[xml(child = "w:right")]
    pub right: Option<RightBorder<'a>>,
    #[xml(child = "w:between")]
    pub between: Option<BetweenBorder<'a>>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:top")]
pub struct TopBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bottom")]
pub struct BottomBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:left")]
pub struct LeftBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:right")]
pub struct RightBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:between")]
pub struct BetweenBorder<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    #[xml(attr = "w:space")]
    pub space: Option<usize>,
    #[xml(attr = "w:sz")]
    pub size: Option<usize>,
    #[xml(attr = "w:val")]
    pub style: Option<BorderStyle>,
}

#[derive(Debug)]
pub enum BorderStyle {
    Single,
    DashDotStroked,
    Dashed,
    DashSmallGap,
    DotDash,
    DotDotDash,
    Dotted,
    Double,
    DoubleWave,
    Inset,
    Nil,
    None,
    Outset,
    Thick,
    ThickThinLargeGap,
    ThickThinMediumGap,
    ThickThinSmallGap,
    ThinThickLargeGap,
    ThinThickMediumGap,
    ThinThickSmallGap,
    ThinThickThinLargeGap,
    ThinThickThinMediumGap,
    ThinThickThinSmallGap,
    ThreeDEmboss,
    ThreeDEngrave,
    Triple,
    Wave,
}

__string_enum! {
    BorderStyle {
        Single = "single",
        DashDotStroked = "dashDotStroked",
        Dashed = "dashed",
        DashSmallGap = "dashSmallGap",
        DotDash = "dotDash",
        DotDotDash = "dotDotDash",
        Dotted = "dotted",
        Double = "double",
        DoubleWave = "doubleWave",
        Inset = "inset",
        Nil = "nil",
        None = "none",
        Outset = "outset",
        Thick = "thick",
        ThickThinLargeGap = "thickThinLargeGap",
        ThickThinMediumGap = "thickThinMediumGap",
        ThickThinSmallGap = "thickThinSmallGap",
        ThinThickLargeGap = "thinThickLargeGap",
        ThinThickMediumGap = "thinThickMediumGap",
        ThinThickSmallGap = "thinThickSmallGap",
        ThinThickThinLargeGap = "thinThickThinLargeGap",
        ThinThickThinMediumGap = "thinThickThinMediumGap",
        ThinThickThinSmallGap = "thinThickThinSmallGap",
        ThreeDEmboss = "threeDEmboss",
        ThreeDEngrave = "threeDEngrave",
        Triple = "triple",
        Wave = "wave",
    }
}

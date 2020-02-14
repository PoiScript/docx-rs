use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __string_enum,
    error::{Error, Result},
};

#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:u")]
pub struct Underline<'a> {
    #[xml(attr = "w:color")]
    pub color: Option<Cow<'a, str>>,
    #[xml(attr = "w:val")]
    pub val: Option<UnderlineStyle>,
}

#[derive(Debug)]
pub enum UnderlineStyle {
    Dash,
    DashDotDotHeavy,
    DashDotHeavy,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDotDash,
    Dotted,
    DottedHeavy,
    Double,
    None,
    Single,
    Thick,
    Wave,
    WavyDouble,
    WavyHeavy,
    Words,
}

__string_enum! {
    UnderlineStyle {
        Dash = "dash",
        DashDotDotHeavy = "dashDotDotHeavy",
        DashDotHeavy = "dashDotHeavy",
        DashedHeavy = "dashedHeavy",
        DashLong = "dashLong",
        DashLongHeavy = "dashLongHeavy",
        DotDash = "dotDash",
        DotDotDash = "dotDotDash",
        Dotted = "dotted",
        DottedHeavy = "dottedHeavy",
        Double = "double",
        None = "none",
        Single = "single",
        Thick = "thick",
        Wave = "wave",
        WavyDouble = "wavyDouble",
        WavyHeavy = "wavyHeavy",
        Words = "words",
    }
}

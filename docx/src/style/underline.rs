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

impl From<String> for Underline<'_> {
    fn from(val: String) -> Self {
        Underline {
            color: Some(val.into()),
            val: None,
        }
    }
}

impl<'a> From<&'a str> for Underline<'a> {
    fn from(val: &'a str) -> Self {
        Underline {
            color: Some(val.into()),
            val: None,
        }
    }
}

impl From<UnderlineStyle> for Underline<'_> {
    fn from(val: UnderlineStyle) -> Self {
        Underline {
            color: None,
            val: Some(val),
        }
    }
}

impl From<(String, UnderlineStyle)> for Underline<'_> {
    fn from(val: (String, UnderlineStyle)) -> Self {
        Underline {
            color: Some(val.0.into()),
            val: Some(val.1),
        }
    }
}

impl<'a> From<(&'a str, UnderlineStyle)> for Underline<'a> {
    fn from(val: (&'a str, UnderlineStyle)) -> Self {
        Underline {
            color: Some(val.0.into()),
            val: Some(val.1),
        }
    }
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

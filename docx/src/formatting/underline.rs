use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __string_enum,
    error::{Error, Result},
};

/// Underline
///
/// ```rust
/// use docx::formatting::*;
///
/// let udl = Underline::from("00ff00");
/// let udl = Underline::from(String::from("ff0000"));
/// let udl = Underline::from(("00ff00", UnderlineStyle::Dash));
/// let udl = Underline::from((String::from("ff0000"), UnderlineStyle::DotDash));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
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
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Underline,
        Underline::default(),
        r#"<w:u/>"#,
        Underline::from("00ff00"),
        r#"<w:u w:color="00ff00"/>"#,
        Underline::from(String::from("ff0000")),
        r#"<w:u w:color="ff0000"/>"#,
        Underline::from(("00ff00", UnderlineStyle::Dash)),
        r#"<w:u w:color="00ff00" w:val="dash"/>"#,
        Underline::from((String::from("ff0000"), UnderlineStyle::DotDash)),
        r#"<w:u w:color="ff0000" w:val="dotDash"/>"#,
    );
}

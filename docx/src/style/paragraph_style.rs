use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
};

use super::{border::Borders, justification::Justification, numbers::Numbers};

/// The root element of a set of paragraph properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct ParagraphStyle<'a> {
    #[xml(child = "w:pStyle")]
    pub name: Option<ParagraphStyleId<'a>>,
    #[xml(child = "w:jc")]
    pub justification: Option<Justification>,
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders<'a>>,
    #[xml(child = "w:numBdr")]
    pub num: Option<Numbers>,
}

impl<'a> ParagraphStyle<'a> {
    __setter!(name: Option<ParagraphStyleId<'a>>);
    __setter!(justification: Option<Justification>);
    __setter!(border: Option<Borders<'a>>);
    __setter!(num: Option<Numbers>);
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pStyle")]
pub struct ParagraphStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for ParagraphStyleId<'a> {
    fn from(val: T) -> Self {
        ParagraphStyleId { value: val.into() }
    }
}

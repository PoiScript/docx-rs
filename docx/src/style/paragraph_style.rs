use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::error::{Error, Result};

use super::{
    border::Borders,
    jc::{Jc, Justification},
    numbers::Numbers,
};

/// The root element of a set of paragraph properties
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:pPr")]
pub struct ParagraphStyle<'a> {
    #[xml(child = "w:pStyle")]
    pub name: Option<ParagraphStyleId<'a>>,
    #[xml(child = "w:jc")]
    pub jc: Option<Jc>,
    #[xml(child = "w:pBdr")]
    pub border: Option<Borders<'a>>,
    #[xml(child = "w:numBdr")]
    pub num: Option<Numbers>,
}

impl<'a> ParagraphStyle<'a> {
    pub fn jc(&mut self, jc: Justification) -> &mut Self {
        self.jc = Some(Jc::new(jc));
        self
    }

    pub fn reset_jc(&mut self) -> &mut Self {
        self.jc = None;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(ParagraphStyleId::new(name.to_owned()));
        self
    }

    pub fn reset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }
}

#[derive(Debug, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:pStyle")]
pub struct ParagraphStyleId<'a> {
    #[xml(attr = "w:val")]
    pub value: Cow<'a, str>,
}

impl<'a> ParagraphStyleId<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(value: S) -> Self {
        ParagraphStyleId {
            value: value.into(),
        }
    }
}

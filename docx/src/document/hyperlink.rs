use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
};

use super::run::Run;

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<Cow<'a, str>>,
    #[xml(child = "w:r")]
    pub content: Run<'a>,
}

impl<'a> Hyperlink<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(anchor: Option<Cow<'a, str>>);
    __setter!(content: Run<'a>);
}

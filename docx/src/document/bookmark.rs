use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::borrow::Cow;

use crate::{
    __setter,
    error::{Error, Result},
};

/// The empty element that defines the beginning of a bookmark
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bookmarkStart")]
pub struct BookmarkStart<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the bookmark name.
    #[xml(attr = "w:name")]
    pub name: Option<Cow<'a, str>>,
}

impl<'a> BookmarkStart<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(name: Option<Cow<'a, str>>);
}

/// The empty element that defines the end of a bookmark
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(leaf, tag = "w:bookmarkEnd")]
pub struct BookmarkEnd<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> BookmarkEnd<'a> {
    __setter!(id: Option<Cow<'a, str>>);
}

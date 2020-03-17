use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, __xml_test_suites};

/// Beginning of bookmark
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:bookmarkStart")]
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

__xml_test_suites!(
    BookmarkStart,
    BookmarkStart::default(),
    r#"<w:bookmarkStart/>"#,
    BookmarkStart::default().id("id"),
    r#"<w:bookmarkStart w:id="id"/>"#,
    BookmarkStart::default().name("name"),
    r#"<w:bookmarkStart w:name="name"/>"#,
);

use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::__setter;

/// Beginning of bookmark
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        BookmarkStart,
        BookmarkStart::default(),
        r#"<w:bookmarkStart/>"#,
        BookmarkStart::default().id(""),
        r#"<w:bookmarkStart w:id=""/>"#,
        BookmarkStart::default().name(""),
        r#"<w:bookmarkStart w:name=""/>"#,
    );
}

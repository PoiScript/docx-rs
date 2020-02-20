use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::__setter;

/// End of bookmark
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(leaf, tag = "w:bookmarkEnd")]
pub struct BookmarkEnd<'a> {
    /// Specifies a unique identifier for the bookmark.
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> BookmarkEnd<'a> {
    __setter!(id: Option<Cow<'a, str>>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        BookmarkEnd,
        BookmarkEnd::default(),
        r#"<w:bookmarkEnd/>"#,
        BookmarkEnd::default().id(""),
        r#"<w:bookmarkEnd w:id=""/>"#,
    );
}

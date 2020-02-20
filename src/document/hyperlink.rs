use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::{__setter, document::Run};

/// The root element of a hyperlink within the paragraph
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink<'a> {
    /// Specifies the ID of the relationship in the relationships part for an external link.
    #[xml(attr = "r:id")]
    pub id: Option<Cow<'a, str>>,
    /// Specifies the name of a bookmark within the document.
    #[xml(attr = "w:anchor")]
    pub anchor: Option<Cow<'a, str>>,
    #[xml(child = "w:r")]
    /// Link content
    pub content: Run<'a>,
}

impl<'a> Hyperlink<'a> {
    __setter!(id: Option<Cow<'a, str>>);
    __setter!(anchor: Option<Cow<'a, str>>);
    __setter!(content: Run<'a>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Hyperlink,
        Hyperlink::default(),
        r#"<w:hyperlink><w:r></w:r></w:hyperlink>"#,
        Hyperlink::default().id(""),
        r#"<w:hyperlink r:id=""><w:r></w:r></w:hyperlink>"#,
        Hyperlink::default().anchor(""),
        r#"<w:hyperlink w:anchor=""><w:r></w:r></w:hyperlink>"#,
    );
}

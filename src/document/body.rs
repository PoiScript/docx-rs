use derive_more::From;
use strong_xml::{XmlRead, XmlWrite};

use crate::document::{Paragraph, Table};

/// Document Body
///
/// This is the main document editing surface.
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:body")]
pub struct Body<'a> {
    /// Specifies the contents of the body of the document.
    #[xml(child = "w:p", child = "w:tbl")]
    pub content: Vec<BodyContent<'a>>,
}

impl<'a> Body<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.content.push(content.into());
        self
    }

    // pub fn iter_text(&self) -> impl Iterator<Item = &Cow<'a, str>> {
    //     self.content
    //         .iter()
    //         .filter_map(|content| match content {
    //             BodyContent::Paragraph(para) => Some(para.iter_text()),
    //         })
    //         .flatten()
    // }

    // pub fn iter_text_mut(&mut self) -> impl Iterator<Item = &mut Cow<'a, str>> {
    //     self.content
    //         .iter_mut()
    //         .filter_map(|content| match content {
    //             BodyContent::Paragraph(para) => Some(para.iter_text_mut()),
    //         })
    //         .flatten()
    // }
}

/// A set of elements that can be contained in the body
#[derive(Debug, From, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BodyContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    #[xml(tag = "w:tbl")]
    Table(Table<'a>),
    // SecProp,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Body,
        Body::default(),
        r#"<w:body></w:body>"#,
        Body {
            content: vec![Paragraph::default().into()]
        },
        r#"<w:body><w:p></w:p></w:body>"#,
        Body {
            content: vec![Table::default().into()]
        },
        r#"<w:body><w:tbl></w:tbl></w:body>"#,
    );
}

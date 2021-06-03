use derive_more::From;
use strong_xml::{XmlRead, XmlWrite};

use crate::__xml_test_suites;
use crate::document::{Paragraph, Table};
use std::ops::{Deref, DerefMut};

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
    pub fn add_content<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.content.push(content.into());
        self
    }
}

impl<'a> Deref for Body<'a> {
    type Target = Vec<BodyContent<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<'a> DerefMut for Body<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
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

__xml_test_suites!(
    Body,
    Body::default(),
    r#"<w:body/>"#,
    Body {
        content: vec![Paragraph::default().into()]
    },
    r#"<w:body><w:p><w:pPr/></w:p></w:body>"#,
    Body {
        content: vec![Table::default().into()]
    },
    r#"<w:body><w:tbl><w:tblPr/></w:tbl></w:body>"#,
);

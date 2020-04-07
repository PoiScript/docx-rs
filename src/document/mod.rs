//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

mod body;
mod bookmark_end;
mod bookmark_start;
mod r#break;
mod grid_column;
mod hyperlink;
mod paragraph;
mod run;
mod table;
mod table_cell;
mod table_grid;
mod table_row;
mod text;

pub use self::{
    body::*, bookmark_end::*, bookmark_start::*, grid_column::*, hyperlink::*, paragraph::*,
    r#break::*, run::*, table::*, table::*, table_cell::*, table_grid::*, table_row::*, text::*,
};

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWriter};

use crate::__xml_test_suites;
use crate::schema::SCHEMA_MAIN;

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:document")]
pub struct Document<'a> {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body<'a>,
}

impl<'a> Document<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.body.push(content);
        self
    }
}

impl<'a> Document<'a> {
    #[cfg(test)]
    pub(crate) fn to_string(&self) -> XmlResult<String> {
        let mut writer = XmlWriter::new(Vec::new());
        self.to_writer(&mut writer)?;
        Ok(String::from_utf8(writer.inner)?)
    }

    pub(crate) fn to_writer<W: Write>(&self, mut writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Document { body } = self;

        log::debug!("[Document] Started writing.");

        writer.write_element_start("w:document")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_element_end_open()?;

        body.to_writer(&mut writer)?;

        writer.write_element_end_close("w:document")?;

        log::debug!("[Document] Finished writing.");

        Ok(())
    }
}

__xml_test_suites!(
    Document,
    Document::default(),
    format!(
        r#"<w:document xmlns:w="{}"><w:body/></w:document>"#,
        SCHEMA_MAIN
    )
    .as_str(),
);

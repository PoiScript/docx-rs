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

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_MAIN,
};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
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

#[inline]
fn document_extend_attrs<W: Write>(_: &Document, mut w: W) -> Result<()> {
    write!(w, r#" xmlns:w="{}""#, SCHEMA_MAIN)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Document,
        Document::default(),
        format!(
            r#"<w:document xmlns:w="{}"><w:body></w:body></w:document>"#,
            SCHEMA_MAIN
        )
        .as_str(),
    );
}

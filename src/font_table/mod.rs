//! Font Table part
//!
//! The corresponding ZIP item is `/word/fontTable.xml`.

mod charset;
mod family;
mod font;
mod pitch;

pub use self::{charset::*, family::*, font::*, pitch::*};

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

use crate::__xml_test_suites;
use crate::schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS};

/// Font Table
///
/// ```rust
/// use docx::font_table::*;
///
/// let fonts = FontTable::default()
///     .push_font("Arial")
///     .push_font(Font::new("Helvetica").family("swiss"));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:fonts")]
#[xml(extend_attrs = "font_table_extend_attrs")]
pub struct FontTable<'a> {
    #[xml(child = "w:font")]
    pub fonts: Vec<Font<'a>>,
}

#[inline]
fn font_table_extend_attrs<W: Write>(_: &FontTable, mut w: W) -> XmlResult<()> {
    write!(&mut w, r#" xmlns:w="{}""#, SCHEMA_MAIN)?;
    write!(&mut w, r#" xmlns:r="{}""#, SCHEMA_RELATIONSHIPS)?;
    Ok(())
}

impl<'a> FontTable<'a> {
    pub fn push_font<T: Into<Font<'a>>>(&mut self, font: T) -> &mut Self {
        self.fonts.push(font.into());
        self
    }
}

__xml_test_suites!(
    FontTable,
    FontTable::default(),
    format!(
        r#"<w:fonts xmlns:w="{}" xmlns:r="{}"/>"#,
        SCHEMA_MAIN, SCHEMA_RELATIONSHIPS
    )
    .as_str(),
    FontTable {
        fonts: vec!["Arial".into()]
    },
    format!(
        r#"<w:fonts xmlns:w="{}" xmlns:r="{}"><w:font w:name="Arial"/></w:fonts>"#,
        SCHEMA_MAIN, SCHEMA_RELATIONSHIPS
    )
    .as_str(),
);

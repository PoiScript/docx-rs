//! Font Table part
//!
//! The corresponding ZIP item is `/word/fontTable.xml`.

mod charset;
mod family;
mod font;
mod pitch;

pub use self::{charset::*, family::*, font::*, pitch::*};

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

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
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:fonts")]
pub struct FontTable<'a> {
    #[xml(child = "w:font")]
    pub fonts: Vec<Font<'a>>,
}

impl<'a> XmlWrite for FontTable<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let FontTable { fonts } = self;

        log::debug!("[FontTable] Started writing.");

        writer.write_element_start("w:fonts")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;
        writer.write_attribute("xmlns:r", SCHEMA_RELATIONSHIPS)?;

        if fonts.is_empty() {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in fonts {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("w:fonts")?;
        }

        log::debug!("[FontTable] Finished writing.");

        Ok(())
    }
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

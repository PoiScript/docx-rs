//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod style;

pub use self::{default_style::*, style::*};

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWriter};

use crate::__xml_test_suites;
use crate::schema::SCHEMA_MAIN;

/// Styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
///
/// ```rust
/// use docx::styles::*;
///
/// let style = Styles::new()
///     .default(DefaultStyle::default())
///     .push(Style::new(StyleType::Paragraph, "style_id"));
/// ```
#[derive(Debug, Default, XmlRead)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:styles")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(default, child = "w:docDefaults")]
    pub default: DefaultStyle<'a>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

impl<'a> Styles<'a> {
    #[cfg(test)]
    pub(crate) fn to_string(&self) -> strong_xml::XmlResult<String> {
        let mut writer = XmlWriter::new(Vec::new());
        self.to_writer(&mut writer)?;
        Ok(String::from_utf8(writer.inner)?)
    }

    pub(crate) fn to_writer<W: Write>(&self, mut writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Styles { default, styles } = self;

        log::debug!("[Styles] Started writing.");

        writer.write_element_start("w:styles")?;

        writer.write_attribute("xmlns:w", SCHEMA_MAIN)?;

        writer.write_element_end_open()?;

        default.to_writer(&mut writer)?;

        for ele in styles {
            ele.to_writer(&mut writer)?;
        }

        writer.write_element_end_close("w:styles")?;

        log::debug!("[Styles] Finished writing.");

        Ok(())
    }
}

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        <Styles as Default>::default()
    }

    pub fn default(&mut self, style: DefaultStyle<'a>) -> &mut Self {
        self.default = style;
        self
    }

    pub fn push(&mut self, style: Style<'a>) -> &mut Self {
        self.styles.push(style);
        self
    }
}

__xml_test_suites!(
    Styles,
    Styles::new(),
    format!(
        r#"<w:styles xmlns:w="{}"><w:docDefaults><w:rPrDefault><w:rPr/></w:rPrDefault><w:pPrDefault><w:pPr/></w:pPrDefault></w:docDefaults></w:styles>"#,
        SCHEMA_MAIN
    )
    .as_str(),
    Styles {
        styles: vec![Style::new(crate::styles::StyleType::Paragraph, "id")],
        ..Default::default()
    },
    format!(
        r#"<w:styles xmlns:w="{}"><w:docDefaults><w:rPrDefault><w:rPr/></w:rPrDefault><w:pPrDefault><w:pPr/></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:styleId="id"><w:pPr/><w:rPr/></w:style></w:styles>"#,
        SCHEMA_MAIN
    )
    .as_str(),
);

//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod style;

pub use self::{default_style::*, style::*};

use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

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
#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(default, child = "w:docDefaults")]
    pub default: DefaultStyle<'a>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

#[inline]
fn styles_extend_attrs<W: Write>(_: &Styles, mut w: W) -> XmlResult<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
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

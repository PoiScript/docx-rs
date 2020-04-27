//! Content-type item
//!
//! The corresponding ZIP item is `/[Content_Types].xml`.

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::schema::SCHEMA_CONTENT_TYPES;

const CONTENT_TYPE_XML: &str = "application/xml";
const CONTENT_TYPE_CORE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const CONTENT_TYPE_RELATIONSHIP: &str = "application/vnd.openxmlformats-package.relationships+xml";
const CONTENT_TYPE_EXTENDED: &str =
    "application/vnd.openxmlformats-officedocument.extended-properties+xml";
const CONTENT_TYPE_DOCUMENT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const CONTENT_TYPE_STYLES: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";

#[derive(Debug, XmlRead)]
#[xml(tag = "Types")]
pub struct ContentTypes<'a> {
    #[xml(child = "Default")]
    pub defaults: Vec<DefaultContentType<'a>>,
    #[xml(child = "Override")]
    pub overrides: Vec<OverrideContentType<'a>>,
}

impl Default for ContentTypes<'static> {
    fn default() -> ContentTypes<'static> {
        ContentTypes {
            defaults: vec![
                DefaultContentType {
                    ext: "rels".into(),
                    ty: CONTENT_TYPE_RELATIONSHIP.into(),
                },
                DefaultContentType {
                    ext: "xml".into(),
                    ty: CONTENT_TYPE_XML.into(),
                },
            ],
            overrides: vec![
                OverrideContentType {
                    part: "/docProps/app.xml".into(),
                    ty: CONTENT_TYPE_EXTENDED.into(),
                },
                OverrideContentType {
                    part: "/docProps/core.xml".into(),
                    ty: CONTENT_TYPE_CORE.into(),
                },
                OverrideContentType {
                    part: "/word/document.xml".into(),
                    ty: CONTENT_TYPE_DOCUMENT.into(),
                },
                OverrideContentType {
                    part: "/word/styles.xml".into(),
                    ty: CONTENT_TYPE_STYLES.into(),
                },
            ],
        }
    }
}

impl<'a> XmlWrite for ContentTypes<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let ContentTypes {
            defaults,
            overrides,
        } = self;

        log::debug!("[ContentTypes] Started writing.");

        writer.write_element_start("Types")?;

        writer.write_attribute("xmlns", SCHEMA_CONTENT_TYPES)?;

        if defaults.is_empty() && overrides.is_empty() {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            for ele in defaults {
                ele.to_writer(writer)?;
            }
            for ele in overrides {
                ele.to_writer(writer)?;
            }
            writer.write_element_end_close("Types")?;
        }

        log::debug!("[ContentTypes] Finished writing.");

        Ok(())
    }
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "Default")]
pub struct DefaultContentType<'a> {
    #[xml(attr = "Extension")]
    pub ext: Cow<'a, str>,
    #[xml(attr = "ContentType")]
    pub ty: Cow<'a, str>,
}

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[xml(tag = "Override")]
pub struct OverrideContentType<'a> {
    #[xml(attr = "PartName")]
    pub part: Cow<'a, str>,
    #[xml(attr = "ContentType")]
    pub ty: Cow<'a, str>,
}

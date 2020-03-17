//! Content-type item
//!
//! The corresponding ZIP item is `/[Content_Types].xml`.

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

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

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Types")]
#[xml(extend_attrs = "content_types_extend_attrs")]
pub struct ContentTypes<'a> {
    #[xml(child = "Default")]
    pub defaults: Vec<DefaultContentType<'a>>,
    #[xml(child = "Override")]
    pub overrides: Vec<OverrideContentType<'a>>,
}

#[inline]
fn content_types_extend_attrs<W: Write>(_: &ContentTypes, mut w: W) -> XmlResult<()> {
    write!(w, " xmlns=\"{}\"", SCHEMA_CONTENT_TYPES)?;
    Ok(())
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

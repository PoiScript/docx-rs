//! Content-type item
//!
//! The corresponding ZIP item is `/[Content_Types].xml`.

use docx_codegen::Xml;
use quick_xml::events::BytesStart;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_CONTENT_TYPES,
};

const CONTENT_TYPE_XML: &str = "application/xml";
const CONTENT_TYPE_CORE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const CONTENT_TYPE_RELATIONSHIP: &str = "application/vnd.openxmlformats-package.relationships+xml";
const CONTENT_TYPE_EXTENDED: &str =
    "application/vnd.openxmlformats-officedocument.extended-properties+xml";
const CONTENT_TYPE_DOCUMENT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const CONTENT_TYPE_STYLES: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";

#[derive(Debug, Xml)]
#[xml(tag = "Types")]
#[xml(extend_attrs = "content_types_extend_attrs")]
pub struct ContentTypes {
    #[xml(child = "Default")]
    pub defaults: Vec<DefaultContentType>,
    #[xml(child = "Override")]
    pub overrides: Vec<OverrideContentType>,
}

#[inline]
fn content_types_extend_attrs(_: &ContentTypes, start: &mut BytesStart) {
    start.push_attribute(("xmlns", SCHEMA_CONTENT_TYPES));
}

impl Default for ContentTypes {
    fn default() -> ContentTypes {
        macro_rules! default_ct {
            ($e:expr, $t:expr) => {
                DefaultContentType {
                    ext: $e.to_string(),
                    ty: $t.to_string(),
                }
            };
        }
        macro_rules! override_ct {
            ($p:expr, $t:expr) => {
                OverrideContentType {
                    part: $p.to_string(),
                    ty: $t.to_string(),
                }
            };
        }
        ContentTypes {
            defaults: vec![
                default_ct!("rels", CONTENT_TYPE_RELATIONSHIP),
                default_ct!("xml", CONTENT_TYPE_XML),
            ],
            overrides: vec![
                override_ct!("/docProps/app.xml", CONTENT_TYPE_EXTENDED),
                override_ct!("/docProps/core.xml", CONTENT_TYPE_CORE),
                override_ct!("/word/document.xml", CONTENT_TYPE_DOCUMENT),
                override_ct!("/word/styles.xml", CONTENT_TYPE_STYLES),
            ],
        }
    }
}

#[derive(Debug, Xml)]
#[xml(tag = "Default")]
#[xml(leaf)]
pub struct DefaultContentType {
    #[xml(attr = "Extension")]
    pub ext: String,
    #[xml(attr = "ContentType")]
    pub ty: String,
}

#[derive(Debug, Xml)]
#[xml(tag = "Override")]
#[xml(leaf)]
pub struct OverrideContentType {
    #[xml(attr = "PartName")]
    pub part: String,
    #[xml(attr = "ContentType")]
    pub ty: String,
}

//! Application-Defined File Properties part
//!
//! The corresponding ZIP item is `/docProps/app.xml`.

use quick_xml::events::BytesStart;
use std::default::Default;

use crate::errors::{Error, Result};
use crate::schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};

#[derive(Debug, Xml)]
#[xml(tag = "Properties")]
#[xml(extend_attrs = "app_extend_attrs")]
pub struct App {
    #[xml(flatten_text = "Template")]
    pub template: Option<String>,
    #[xml(flatten_text = "TotalTime")]
    pub total_time: Option<String>,
    #[xml(flatten_text = "Pages")]
    pub pages: Option<String>,
    #[xml(flatten_text = "Words")]
    pub words: Option<String>,
    #[xml(flatten_text = "Characters")]
    pub characters: Option<String>,
    #[xml(flatten_text = "Application")]
    pub application: Option<String>,
    #[xml(flatten_text = "DocSecurity")]
    pub doc_security: Option<String>,
    #[xml(flatten_text = "Lines")]
    pub lines: Option<String>,
    #[xml(flatten_text = "Paragraphs")]
    pub paragraphs: Option<String>,
    #[xml(flatten_text = "ScaleCrop")]
    pub scale_crop: Option<String>,
    #[xml(flatten_text = "Company")]
    pub company: Option<String>,
    #[xml(flatten_text = "LinksUpToDate")]
    pub links_up_to_date: Option<String>,
    #[xml(flatten_text = "CharactersWithSpaces")]
    pub characters_with_spaces: Option<String>,
    #[xml(flatten_text = "SharedDoc")]
    pub shared_doc: Option<String>,
    #[xml(flatten_text = "HyperlinksChanged")]
    pub hyperlinks_changed: Option<String>,
    #[xml(flatten_text = "AppVersion")]
    pub app_version: Option<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            template: Some("Normal.dotm".to_string()),
            total_time: Some("1".to_string()),
            pages: Some("1".to_string()),
            words: Some("0".to_string()),
            characters: Some("0".to_string()),
            application: Some("docx-rs".to_string()),
            doc_security: Some("0".to_string()),
            lines: Some("0".to_string()),
            paragraphs: Some("1".to_string()),
            scale_crop: Some("false".to_string()),
            company: Some("MS".to_string()),
            links_up_to_date: Some("false".to_string()),
            characters_with_spaces: Some("25".to_string()),
            shared_doc: Some("false".to_string()),
            hyperlinks_changed: Some("false".to_string()),
            app_version: Some("12.0000".to_string()),
        }
    }
}

#[inline]
fn app_extend_attrs(_: &App, start: &mut BytesStart) {
    start.push_attribute(("xmlns", SCHEMAS_EXTENDED));
    start.push_attribute(("xmlns:vt", SCHEMA_DOC_PROPS_V_TYPES));
}

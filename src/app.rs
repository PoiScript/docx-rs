//! Application-Defined File Properties part
//!
//! The corresponding ZIP item is `/docProps/app.xml`.

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite};

use crate::schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};

#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Properties")]
#[xml(extend_attrs = "app_extend_attrs")]
pub struct App<'a> {
    #[xml(flatten_text = "Template")]
    pub template: Option<Cow<'a, str>>,
    #[xml(flatten_text = "TotalTime")]
    pub total_time: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Pages")]
    pub pages: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Words")]
    pub words: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Characters")]
    pub characters: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Application")]
    pub application: Option<Cow<'a, str>>,
    #[xml(flatten_text = "DocSecurity")]
    pub doc_security: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Lines")]
    pub lines: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Paragraphs")]
    pub paragraphs: Option<Cow<'a, str>>,
    #[xml(flatten_text = "ScaleCrop")]
    pub scale_crop: Option<Cow<'a, str>>,
    #[xml(flatten_text = "Company")]
    pub company: Option<Cow<'a, str>>,
    #[xml(flatten_text = "LinksUpToDate")]
    pub links_up_to_date: Option<Cow<'a, str>>,
    #[xml(flatten_text = "CharactersWithSpaces")]
    pub characters_with_spaces: Option<Cow<'a, str>>,
    #[xml(flatten_text = "SharedDoc")]
    pub shared_doc: Option<Cow<'a, str>>,
    #[xml(flatten_text = "HyperlinksChanged")]
    pub hyperlinks_changed: Option<Cow<'a, str>>,
    #[xml(flatten_text = "AppVersion")]
    pub app_version: Option<Cow<'a, str>>,
}

#[inline]
fn app_extend_attrs<W: Write>(_: &App, mut w: W) -> XmlResult<()> {
    write!(&mut w, r#" xmlns="{}""#, SCHEMAS_EXTENDED)?;
    write!(&mut w, r#" xmlns:vt="{}""#, SCHEMA_DOC_PROPS_V_TYPES)?;
    Ok(())
}

impl Default for App<'static> {
    fn default() -> App<'static> {
        App {
            template: Some("Normal.dotm".into()),
            total_time: Some("1".into()),
            pages: Some("1".into()),
            words: Some("0".into()),
            characters: Some("0".into()),
            application: Some("docx-rs".into()),
            doc_security: Some("0".into()),
            lines: Some("0".into()),
            paragraphs: Some("1".into()),
            scale_crop: Some("false".into()),
            company: Some("MS".into()),
            links_up_to_date: Some("false".into()),
            characters_with_spaces: Some("25".into()),
            shared_doc: Some("false".into()),
            hyperlinks_changed: Some("false".into()),
            app_version: Some("12.0000".into()),
        }
    }
}

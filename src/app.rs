//! Application-Defined File Properties part
//!
//! The corresponding ZIP item is `/docProps/app.xml`.

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWrite, XmlWriter};

use crate::schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};

#[derive(Debug, XmlRead)]
#[xml(tag = "Properties")]
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

impl<'a> XmlWrite for App<'a> {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let App {
            template,
            total_time,
            pages,
            words,
            characters,
            application,
            doc_security,
            lines,
            paragraphs,
            scale_crop,
            company,
            links_up_to_date,
            characters_with_spaces,
            shared_doc,
            hyperlinks_changed,
            app_version,
        } = self;

        log::debug!("[App] Started writing.");

        writer.write_element_start("Properties")?;

        writer.write_attribute("xmlns", SCHEMAS_EXTENDED)?;
        writer.write_attribute("xmlns:vt", SCHEMA_DOC_PROPS_V_TYPES)?;

        if template.is_none()
            && total_time.is_none()
            && pages.is_none()
            && words.is_none()
            && characters.is_none()
            && application.is_none()
            && doc_security.is_none()
            && lines.is_none()
            && paragraphs.is_none()
            && scale_crop.is_none()
            && company.is_none()
            && links_up_to_date.is_none()
            && characters_with_spaces.is_none()
            && shared_doc.is_none()
            && hyperlinks_changed.is_none()
            && app_version.is_none()
        {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            if let Some(val) = template {
                writer.write_flatten_text("Template", val, true)?;
            }
            if let Some(val) = total_time {
                writer.write_flatten_text("TotalTime", val, true)?;
            }
            if let Some(val) = pages {
                writer.write_flatten_text("Pages", val, true)?;
            }
            if let Some(val) = words {
                writer.write_flatten_text("Words", val, true)?;
            }
            if let Some(val) = characters {
                writer.write_flatten_text("Characters", val, true)?;
            }
            if let Some(val) = application {
                writer.write_flatten_text("Application", val, true)?;
            }
            if let Some(val) = doc_security {
                writer.write_flatten_text("DocSecurity", val, true)?;
            }
            if let Some(val) = lines {
                writer.write_flatten_text("Lines", val, true)?;
            }
            if let Some(val) = paragraphs {
                writer.write_flatten_text("Paragraphs", val, true)?;
            }
            if let Some(val) = scale_crop {
                writer.write_flatten_text("ScaleCrop", val, true)?;
            }
            if let Some(val) = company {
                writer.write_flatten_text("Company", val, true)?;
            }
            if let Some(val) = links_up_to_date {
                writer.write_flatten_text("LinksUpToDate", val, true)?;
            }
            if let Some(val) = characters_with_spaces {
                writer.write_flatten_text("CharactersWithSpaces", val, true)?;
            }
            if let Some(val) = shared_doc {
                writer.write_flatten_text("SharedDoc", val, true)?;
            }
            if let Some(val) = hyperlinks_changed {
                writer.write_flatten_text("HyperlinksChanged", val, true)?;
            }
            if let Some(val) = app_version {
                writer.write_flatten_text("AppVersion", val, true)?;
            }
            writer.write_element_end_close("Properties")?;
        }

        log::debug!("[App] Finished writing.");

        Ok(())
    }
}

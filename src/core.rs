//! Core File Properties part
//!
//! The corresponding ZIP item is `/docProps/core.xml`.

use std::borrow::Cow;
use std::io::Write;
use strong_xml::{XmlRead, XmlResult, XmlWriter};

use crate::schema::SCHEMA_CORE;

#[derive(Debug, Default, XmlRead)]
#[xml(tag = "cp:coreProperties")]
pub struct Core<'a> {
    #[xml(flatten_text = "dc:title")]
    pub title: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:subject")]
    pub subject: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:creator")]
    pub creator: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:keywords")]
    pub keywords: Option<Cow<'a, str>>,
    #[xml(flatten_text = "dc:description")]
    pub description: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:lastModifiedBy")]
    pub last_modified_by: Option<Cow<'a, str>>,
    #[xml(flatten_text = "cp:revision")]
    pub revision: Option<Cow<'a, str>>,
}

impl<'a> Core<'a> {
    pub(crate) fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        let Core {
            title,
            subject,
            creator,
            keywords,
            description,
            last_modified_by,
            revision,
        } = self;

        log::debug!("[Core] Started writing.");

        writer.write_element_start("cp:coreProperties")?;

        writer.write_attribute("xmlns:cp", SCHEMA_CORE)?;

        if title.is_none()
            && subject.is_none()
            && creator.is_none()
            && keywords.is_none()
            && description.is_none()
            && last_modified_by.is_none()
            && revision.is_none()
        {
            writer.write_element_end_empty()?;
        } else {
            writer.write_element_end_open()?;
            if let Some(val) = title {
                writer.write_flatten_text("dc:title", val)?;
            }
            if let Some(val) = subject {
                writer.write_flatten_text("dc:subject", val)?;
            }
            if let Some(val) = creator {
                writer.write_flatten_text("dc:creator", val)?;
            }
            if let Some(val) = keywords {
                writer.write_flatten_text("cp:keywords", val)?;
            }
            if let Some(val) = description {
                writer.write_flatten_text("dc:description", val)?;
            }
            if let Some(val) = last_modified_by {
                writer.write_flatten_text("cp:lastModifiedBy", val)?;
            }
            if let Some(val) = revision {
                writer.write_flatten_text("cp:revision", val)?;
            }
            writer.write_element_end_close("cp:coreProperties")?;
        }

        log::debug!("[Core] Finished writing.");

        Ok(())
    }
}

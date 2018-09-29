//! Core File Properties part
//!
//! The corresponding ZIP item is `/docProps/core.xml`.

use quick_xml::events::BytesStart;
use std::borrow::Cow;

use errors::{Error, Result};
use schema::SCHEMA_CORE;

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "cp:coreProperties")]
#[xml(extend_attrs = "core_extend_attrs")]
pub struct Core<'a> {
  #[xml(flatten_text)]
  #[xml(tag = "dc:title")]
  pub title: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:subject")]
  pub subject: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:creator")]
  pub creator: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:keywords")]
  pub keywords: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:description")]
  pub description: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:lastModifiedBy")]
  pub last_modified_by: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:revision")]
  pub revision: Option<Cow<'a, str>>,
}

fn core_extend_attrs(_: &Core, start: &mut BytesStart) {
  start.push_attribute(("xmlns:cp", SCHEMA_CORE));
}

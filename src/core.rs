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
  title: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:subject")]
  subject: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:creator")]
  creator: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:keywords")]
  keywords: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "dc:description")]
  description: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:lastModifiedBy")]
  last_modified_by: Cow<'a, str>,
  #[xml(flatten_text)]
  #[xml(tag = "cp:revision")]
  revision: Cow<'a, str>,
}

fn core_extend_attrs(_: &Core, start: &mut BytesStart) {
  start.push_attribute(("xmlns:cp", SCHEMA_CORE));
}

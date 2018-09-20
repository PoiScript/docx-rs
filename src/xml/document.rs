use quick_xml::events::BytesStart;

use body::Body;
use errors::{Error, Result};
use schema::SCHEMA_MAIN;
use xml::Xml;

#[derive(Debug, Default, Xml)]
#[xml(event = "Start")]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct DocumentXml<'a> {
  #[xml(child)]
  #[xml(tag = "w:body")]
  pub body: Body<'a>,
}

fn document_extend_attrs(_: &DocumentXml, start: &mut BytesStart) {
  start.push_attribute(("xmlns:w", SCHEMA_MAIN));
}

use quick_xml::events::BytesStart;
use std::borrow::Cow;
use std::default::Default;

use errors::{Error, Result};
use schema::SCHEMA_CONTENT_TYPES;

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
#[xml(event = "Start")]
#[xml(tag = "Types")]
#[xml(extend_attrs = "content_types_extend_attrs")]
pub struct ContentTypes<'a> {
  #[xml(child)]
  #[xml(tag = "Default")]
  defaults: Vec<DefaultContentType<'a>>,
  #[xml(child)]
  #[xml(tag = "Override")]
  overrides: Vec<OverrideContentType<'a>>,
}

fn content_types_extend_attrs(_: &ContentTypes, start: &mut BytesStart) {
  start.push_attribute(("xmlns", SCHEMA_CONTENT_TYPES));
}

impl<'a> Default for ContentTypes<'a> {
  fn default() -> ContentTypes<'a> {
    macro_rules! default_ct {
      ($e:expr, $t:expr) => {
        DefaultContentType {
          ext: Cow::Borrowed($e),
          ty: Cow::Borrowed($t),
        }
      };
    }
    macro_rules! override_ct {
      ($p:expr, $t:expr) => {
        OverrideContentType {
          part: Cow::Borrowed($p),
          ty: Cow::Borrowed($t),
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
#[xml(event = "Empty")]
#[xml(tag = "Default")]
struct DefaultContentType<'a> {
  #[xml(attr = "Extension")]
  ext: Cow<'a, str>,
  #[xml(attr = "ContentType")]
  ty: Cow<'a, str>,
}

#[derive(Debug, Xml)]
#[xml(event = "Empty")]
#[xml(tag = "Override")]
struct OverrideContentType<'a> {
  #[xml(attr = "PartName")]
  part: Cow<'a, str>,
  #[xml(attr = "ContentType")]
  ty: Cow<'a, str>,
}

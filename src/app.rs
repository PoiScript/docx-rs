use quick_xml::events::BytesStart;
use std::borrow::Cow;
use std::default::Default;

use errors::{Error, Result};
use schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "Properties")]
#[xml(extend_attrs = "app_extend_attrs")]
pub struct App<'a> {
  #[xml(flatten_text)]
  #[xml(tag = "Tempalte")]
  pub template: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "TotalTime")]
  pub total_time: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Pages")]
  pub pages: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Words")]
  pub words: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Characters")]
  pub characters: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Application")]
  pub application: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "DocSecurity")]
  pub doc_security: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Lines")]
  pub lines: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Paragraphs")]
  pub paragraphs: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "ScaleCrop")]
  pub scale_crop: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "Company")]
  pub company: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "LinksUpToDate")]
  pub links_up_to_date: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "CharactersWithSpaces")]
  pub characters_with_spaces: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "SharedDoc")]
  pub shared_doc: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "HyperlinksChanged")]
  pub hyperlinks_changed: Option<Cow<'a, str>>,
  #[xml(flatten_text)]
  #[xml(tag = "AppVersion")]
  pub app_version: Option<Cow<'a, str>>,
}

impl<'a> Default for App<'a> {
  fn default() -> App<'a> {
    App {
      template: Some(Cow::Borrowed("Normal.dotm")),
      total_time: Some(Cow::Borrowed("1")),
      pages: Some(Cow::Borrowed("1")),
      words: Some(Cow::Borrowed("0")),
      characters: Some(Cow::Borrowed("0")),
      application: Some(Cow::Borrowed("docx-rs")),
      doc_security: Some(Cow::Borrowed("0")),
      lines: Some(Cow::Borrowed("0")),
      paragraphs: Some(Cow::Borrowed("1")),
      scale_crop: Some(Cow::Borrowed("false")),
      company: Some(Cow::Borrowed("MS")),
      links_up_to_date: Some(Cow::Borrowed("false")),
      characters_with_spaces: Some(Cow::Borrowed("25")),
      shared_doc: Some(Cow::Borrowed("false")),
      hyperlinks_changed: Some(Cow::Borrowed("false")),
      app_version: Some(Cow::Borrowed("12.0000")),
    }
  }
}

fn app_extend_attrs(_: &App, start: &mut BytesStart) {
  start.push_attribute(("xmlns", SCHEMAS_EXTENDED));
  start.push_attribute(("xmlns:vt", SCHEMA_DOC_PROPS_V_TYPES));
}

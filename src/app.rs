use quick_xml::events::BytesStart;
use std::borrow::Cow;
use std::default::Default;

use errors::{Error, Result};
use schema::{SCHEMAS_EXTENDED, SCHEMA_DOC_PROPS_V_TYPES};
use Xml;

#[derive(Debug, Xml)]
#[xml(event = "Start")]
#[xml(tag = "Properties")]
#[xml(extend_attrs = "app_extend_attrs")]
pub struct App<'a> {
  #[xml(flattern_text)]
  #[xml(tag = "Tempalte")]
  template: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "TotalTime")]
  total_time: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Pages")]
  pages: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Words")]
  words: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Characters")]
  characters: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Application")]
  application: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "DocSecurity")]
  doc_security: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Lines")]
  lines: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Paragraphs")]
  paragraphs: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "ScaleCrop")]
  scale_crop: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "Company")]
  company: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "LinksUpToDate")]
  links_up_to_date: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "CharactersWithSpaces")]
  characters_with_spaces: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "SharedDoc")]
  shared_doc: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "HyperlinksChanged")]
  hyperlinks_changed: Cow<'a, str>,
  #[xml(flattern_text)]
  #[xml(tag = "AppVersion")]
  app_version: Cow<'a, str>,
}

impl<'a> Default for App<'a> {
  fn default() -> App<'a> {
    App {
      template: Cow::Borrowed("Normal.dotm"),
      total_time: Cow::Borrowed("1"),
      pages: Cow::Borrowed("1"),
      words: Cow::Borrowed("0"),
      characters: Cow::Borrowed("0"),
      application: Cow::Borrowed("docx-rs"),
      doc_security: Cow::Borrowed("0"),
      lines: Cow::Borrowed("0"),
      paragraphs: Cow::Borrowed("1"),
      scale_crop: Cow::Borrowed("false"),
      company: Cow::Borrowed("MS"),
      links_up_to_date: Cow::Borrowed("false"),
      characters_with_spaces: Cow::Borrowed("25"),
      shared_doc: Cow::Borrowed("false"),
      hyperlinks_changed: Cow::Borrowed("false"),
      app_version: Cow::Borrowed("12.0000"),
    }
  }
}

fn app_extend_attrs(_: &App, start: &mut BytesStart) {
  start.push_attribute(("xmlns", SCHEMAS_EXTENDED));
  start.push_attribute(("xmlns:vt", SCHEMA_DOC_PROPS_V_TYPES));
}

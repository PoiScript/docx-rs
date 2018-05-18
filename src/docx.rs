use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;
use std::io::{Seek, Write};
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use body::Para;
use schema::{SCHEMA_CORE, SCHEMA_REL_EXTENDED};
use xml::{AppXml, ContentTypesXml, CoreXml, DocumentXml, FontTableXml, RelsXml, Xml};

static APP_XML: &'static str = "docProps/app.xml";
static CONTENT_TYPES_XML: &'static str = "[Content_Types].xml";
static CORE_XML: &'static str = "docProps/core.xml";
static DOCUMENT_XML: &'static str = "word/document.xml";
static FONT_TABLE_XML: &'static str = "word/fontTable.xml";

static RELS: &'static str = "_rels/.rels";
//static DOCUMENT_RELS: &'static str = "word/_rels/document.xml.rels";

#[derive(Debug, Default)]
pub struct Docx<'a> {
  app_xml: Option<AppXml<'a>>,
  core_xml: Option<CoreXml<'a>>,
  content_types_xml: ContentTypesXml<'a>,
  document_xml: DocumentXml<'a>,
  font_table_xml: Option<FontTableXml<'a>>,
  rels: RelsXml<'a>,
}

impl<'a> Docx<'a> {
  pub fn new() -> Docx<'a> {
    Docx {
      app_xml: None,
      core_xml: None,
      content_types_xml: ContentTypesXml::default(),
      document_xml: DocumentXml::default(),
      font_table_xml: None,
      rels: RelsXml::default(),
    }
  }

  pub fn append_para(&mut self, para: Para<'a>) {
    self.document_xml.add_para(para);
  }

  pub fn generate<T: Write + Seek>(&mut self, writer: T) -> ZipResult<()> {
    let mut zip = ZipWriter::new(writer);
    let opt = FileOptions::default()
      .compression_method(CompressionMethod::Deflated)
      .unix_permissions(0o755);

    macro_rules! write {
      ($xml:expr, $name:ident) => {{
        zip.start_file($name, opt)?;
        let mut writer = Writer::new(zip);
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)));
        $xml.write(&mut writer);
        zip = writer.into_inner();
      }};
    }

    if let Some(app_xml) = &self.app_xml {
      write!(app_xml, APP_XML);
      self.rels.add_rel((SCHEMA_REL_EXTENDED, APP_XML));
    }

    if let Some(core_xml) = &self.core_xml {
      write!(core_xml, CORE_XML);
      self.rels.add_rel((SCHEMA_CORE, CORE_XML));
    }

    write!(self.content_types_xml, CONTENT_TYPES_XML);

    write!(self.document_xml, DOCUMENT_XML);

    if let Some(font_table_xml) = &self.font_table_xml {
      write!(font_table_xml, FONT_TABLE_XML);
    }

    write!(self.rels, RELS);

    zip.finish()?;

    Result::Ok(())
  }
}

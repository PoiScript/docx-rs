use std::io::{Seek, Write};
use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use xml::{AppXml, ContentTypesXml, CoreXml, DocumentXml, RelsXml, Xml};

static APP_XML: &'static str = "docProps/app.xml";
static CONTENT_TYPES_XML: &'static str = "[Content_Types].xml";
static CORE_XML: &'static str = "docProps/core.xml";
static DOCUMENT_XML: &'static str = "word/document.xml";

static RELS: &'static str = "_rels/.rels";
//static DOCUMENT_RELS: &'static str = "word/_rels/document.xml.rels";

pub struct Docx<'a> {
  app_xml: AppXml<'a>,
  content_types_xml: ContentTypesXml<'a>,
  core_xml: CoreXml<'a>,
  document_xml: DocumentXml<'a>,
  rels: RelsXml<'a>,
}

impl<'a> Docx<'a> {
  pub fn new() -> Docx<'a> {
    Docx {
      app_xml: AppXml::default(),
      content_types_xml: ContentTypesXml::default(),
      core_xml: CoreXml::default(),
      document_xml: DocumentXml::default(),
      rels: RelsXml::default(),
    }
  }

  pub fn generate<T: Write + Seek>(&self, writer: T) -> ZipResult<()> {
    let mut zip = ZipWriter::new(writer);
    let opt = FileOptions::default()
      .compression_method(CompressionMethod::Deflated)
      .unix_permissions(0o755);

    zip.start_file(APP_XML, opt)?;
    zip.write_all(&self.app_xml.generate())?;

    zip.start_file(CONTENT_TYPES_XML, opt)?;
    zip.write_all(&self.content_types_xml.generate())?;

    zip.start_file(CORE_XML, opt)?;
    zip.write_all(&self.core_xml.generate())?;

    zip.start_file(DOCUMENT_XML, opt)?;
    zip.write_all(&self.document_xml.generate())?;

    zip.start_file(RELS, opt)?;
    zip.write_all(&self.rels.generate())?;

    zip.finish()?;

    Result::Ok(())
  }
}

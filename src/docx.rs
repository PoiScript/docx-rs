use quick_xml::events::*;
use quick_xml::Writer;
use std::default::Default;

use quick_xml::Reader;
use std::io::{BufReader, Read, Seek, Write};
use zip::result::ZipError;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::{ZipArchive, ZipWriter};

use body::Para;
use errors::{Error, Result};
use schema::{
  SCHEMA_CORE, SCHEMA_FONT_TABLE, SCHEMA_OFFICE_DOCUMENT, SCHEMA_REL_EXTENDED, SCHEMA_STYLES,
};
use style::Style;
use xml::{AppXml, ContentTypes, CoreXml, DocumentXml, FontTableXml, RelsXml, StylesXml, Xml};

#[derive(Debug, Default)]
pub struct Docx<'a> {
  app_xml: Option<AppXml<'a>>,
  core_xml: Option<CoreXml<'a>>,
  content_types_xml: ContentTypes<'a>,
  document_xml: DocumentXml<'a>,
  font_table_xml: Option<FontTableXml<'a>>,
  styles_xml: Option<StylesXml<'a>>,
  rels: RelsXml<'a>,
  document_rels: Option<RelsXml<'a>>,
}

impl<'a> Docx<'a> {
  pub fn create_para(&mut self) -> &mut Para<'a> {
    self.document_xml.body.create_para()
  }

  pub fn create_style(&mut self) -> &mut Style<'a> {
    self
      .styles_xml
      .get_or_insert(StylesXml::default())
      .create_style()
  }

  pub fn generate<T: Write + Seek>(&mut self, writer: T) -> Result<T> {
    let mut zip = ZipWriter::new(writer);
    let opt = FileOptions::default()
      .compression_method(CompressionMethod::Deflated)
      .unix_permissions(0o755);

    macro_rules! write {
      ($xml:expr, $name:tt) => {{
        zip.start_file($name, opt)?;
        let mut writer = Writer::new(zip);
        writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)))?;
        $xml.write(&mut writer)?;
        zip = writer.into_inner();
      }};

      ($xml:expr, $name:tt, $rel:expr, $schema:expr, $target:tt) => {{
        write!($xml, $name);
        $rel.add_rel($schema, $target);
      }};
    }

    macro_rules! option_write {
      ($xml:expr, $($rest:tt)*) => {{
        if let Some(ref xml) = $xml {
          write!(xml, $($rest)*);
        }
      }};
    }

    // content types
    write!(self.content_types_xml, "[Content_Types].xml");

    // document properties
    option_write!(
      self.app_xml,
      "docProps/app.xml",
      self.rels,
      SCHEMA_REL_EXTENDED,
      "docProps/app.xml"
    );
    option_write!(
      self.core_xml,
      "docProps/core.xml",
      self.rels,
      SCHEMA_CORE,
      "docProps/core.xml"
    );

    // documents specific parts
    write!(
      self.document_xml,
      "word/document.xml",
      self.rels, SCHEMA_OFFICE_DOCUMENT, "word/document.xml"
    );
    option_write!(
      self.styles_xml,
      "word/styles.xml",
      self.document_rels.get_or_insert(RelsXml::default()),
      SCHEMA_STYLES,
      "styles.xml"
    );
    option_write!(
      self.font_table_xml,
      "word/fontTable.xml",
      self.document_rels.get_or_insert(RelsXml::default()),
      SCHEMA_FONT_TABLE,
      "fontTable.xml"
    );

    // relationships
    write!(self.rels, "_rels/.rels");
    option_write!(self.document_rels, "word/_rels/document.xml.rels");

    Ok(zip.finish()?)
  }

  pub fn parse<T: Read + Seek, 'a>(reader: T) -> Result<Docx<'a>> {
    let mut zip = ZipArchive::new(reader).unwrap();

    macro_rules! read {
      ($xml:tt, $name:expr) => {{
        let file = zip.by_name($name)?;
        let mut reader = Reader::from_reader(BufReader::new(file));
        reader.trim_text(true);
        $xml::read(&mut reader, None)?
      }};
    }

    macro_rules! option_read {
      ($xml:tt, $name:expr) => {
        match zip.by_name($name) {
          Err(ZipError::FileNotFound) => None,
          Err(e) => return Err(Error::Zip(e)),
          Ok(file) => {
            let mut reader = Reader::from_reader(BufReader::new(file));
            reader.trim_text(true);
            Some($xml::read(&mut reader, None)?)
          }
        }
      };
    }

    Ok(Docx {
      app_xml: option_read!(AppXml, "docProps/app.xml"),
      content_types_xml: read!(ContentTypes, "[Content_Types].xml"),
      core_xml: option_read!(CoreXml, "docProps/core.xml"),
      document_rels: option_read!(RelsXml, "word/_rels/document.xml.rels"),
      document_xml: read!(DocumentXml, "word/document.xml"),
      font_table_xml: option_read!(FontTableXml, "word/fontTable.xml"),
      rels: read!(RelsXml, "_rels/.rels"),
      styles_xml: option_read!(StylesXml, "word/styles.xml"),
    })
  }
}

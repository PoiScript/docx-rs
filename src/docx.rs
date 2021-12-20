use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use strong_xml::{XmlRead, XmlWrite, XmlWriter};
use zip::{result::ZipError, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    app::App,
    content_type::ContentTypes,
    core::Core,
    document::Document,
    error::DocxResult,
    font_table::FontTable,
    rels::Relationships,
    schema::{
        SCHEMA_CORE, SCHEMA_FONT_TABLE, SCHEMA_OFFICE_DOCUMENT, SCHEMA_REL_EXTENDED, SCHEMA_STYLES,
    },
    styles::Styles,
};

/// A WordprocessingML package
#[derive(Debug, Default)]
pub struct Docx<'a> {
    /// Specifies package-level properties part
    pub app: Option<App<'a>>,
    /// Specifies core properties part
    pub core: Option<Core<'a>>,
    /// Specifies the content type of relationship parts and the main document part.
    pub content_types: ContentTypes<'a>,
    /// Specifies the main document part.
    pub document: Document<'a>,
    /// Specifies the font table part
    pub font_table: Option<FontTable<'a>>,
    /// Specifies the style definitions part
    pub styles: Styles<'a>,
    /// Specifies the package-level relationship to the main document part
    pub rels: Relationships<'a>,
    /// Specifies the part-level relationship to the main document part
    pub document_rels: Option<Relationships<'a>>,
}

impl<'a> Docx<'a> {
    pub fn write<W: Write + Seek>(&mut self, writer: W) -> DocxResult<W> {
        let mut writer = XmlWriter::new(ZipWriter::new(writer));

        let opt = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // ==== Add Relationships ====

        if self.app.is_some() {
            self.rels.add_rel(SCHEMA_REL_EXTENDED, "docProps/app.xml");
        }

        if self.core.is_some() {
            self.rels.add_rel(SCHEMA_CORE, "docProps/core.xml");
        }

        self.rels
            .add_rel(SCHEMA_OFFICE_DOCUMENT, "word/document.xml");

        self.document_rels
            .get_or_insert(Relationships::default())
            .add_rel(SCHEMA_STYLES, "styles.xml");

        if self.font_table.is_some() {
            self.document_rels
                .get_or_insert(Relationships::default())
                .add_rel(SCHEMA_FONT_TABLE, "fontTable.xml");
        }

        // ==== Write Zip Item ====

        macro_rules! write_xml {
            (Some($xml:expr) => $name:tt) => {
                if let Some(ref xml) = $xml {
                    write_xml!(xml => $name);
                }
            };
            (Some($xml:expr) => $name:tt $($rest:tt)*) => {
                write_xml!(Some($xml) => $name);
                write_xml!($($rest)*);
            };
            ($xml:expr => $name:tt) => {
                writer.inner.start_file($name, opt)?;
                $xml.to_writer(&mut writer)?;
            };
            ($xml:expr => $name:tt $($rest:tt)*) => {
                write_xml!($xml => $name);
                write_xml!($($rest)*);
            };
        }

        write_xml!(
            self.content_types        => "[Content_Types].xml"
            Some(self.app)            => "docProps/app.xml"
            Some(self.core)           => "docProps/core.xml"
            self.rels                 => "_rels/.rels"
            self.document             => "word/document.xml"
            self.styles               => "word/styles.xml"
            Some(self.font_table)     => "word/fontTable.xml"
            Some(self.document_rels)  => "word/_rels/document.xml.rels"
        );

        Ok(writer.inner.finish()?)
    }

    pub fn write_file<P: AsRef<Path>>(&mut self, path: P) -> DocxResult<File> {
        let file = File::create(path)?;
        self.write(file)
    }
}

/// An extracted docx file
pub struct DocxFile {
    app: Option<String>,
    content_types: String,
    core: Option<String>,
    document: String,
    document_rels: Option<String>,
    font_table: Option<String>,
    rels: String,
    styles: Option<String>,
}

impl DocxFile {
    /// Extracts from reader
    pub fn from_reader<T: Read + Seek>(reader: T) -> DocxResult<Self> {
        let mut zip = ZipArchive::new(reader)?;

        macro_rules! read {
            ($xml:tt, $name:expr) => {{
                let mut file = zip.by_name($name)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                buffer
            }};
        }

        macro_rules! option_read {
            ($xml:tt, $name:expr) => {
                match zip.by_name($name) {
                    Err(ZipError::FileNotFound) => None,
                    Err(e) => return Err(e.into()),
                    Ok(mut file) => {
                        let mut buffer = String::new();
                        file.read_to_string(&mut buffer)?;
                        Some(buffer)
                    }
                }
            };
        }

        let app = option_read!(App, "docProps/app.xml");
        let content_types = read!(ContentTypes, "[Content_Types].xml");
        let core = option_read!(Core, "docProps/core.xml");
        let document_rels = option_read!(Relationships, "word/_rels/document.xml.rels");
        let document = read!(Document, "word/document.xml");
        let font_table = option_read!(FontTable, "word/fontTable.xml");
        let rels = read!(Relationships, "_rels/.rels");
        let styles = option_read!(Styles, "word/styles.xml");

        Ok(DocxFile {
            app,
            content_types,
            core,
            document_rels,
            document,
            font_table,
            rels,
            styles,
        })
    }

    /// Extracts from file
    #[inline]
    pub fn from_file<P: AsRef<Path>>(path: P) -> DocxResult<Self> {
        Self::from_reader(File::open(path)?)
    }

    /// Parses content into `Docx` struct
    pub fn parse<'a>(&'a self) -> DocxResult<Docx<'a>> {
        let app = if let Some(content) = &self.app {
            Some(App::from_str(content)?)
        } else {
            None
        };

        let document = Document::from_str(&self.document)?;

        let content_types = ContentTypes::from_str(&self.content_types)?;

        let core = if let Some(content) = &self.core {
            Some(Core::from_str(content)?)
        } else {
            None
        };

        let document_rels = if let Some(content) = &self.document_rels {
            Some(Relationships::from_str(content)?)
        } else {
            None
        };

        let font_table = if let Some(content) = &self.font_table {
            Some(FontTable::from_str(content)?)
        } else {
            None
        };

        let rels = Relationships::from_str(&self.rels)?;

        let styles = self
            .styles
            .as_ref()
            .map(|content| Styles::from_str(&content))
            .transpose()?
            .unwrap_or_default();

        Ok(Docx {
            app,
            content_types,
            core,
            document,
            document_rels,
            font_table,
            rels,
            styles,
        })
    }
}

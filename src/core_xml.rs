/// docProps/core.xml

use std::collections::LinkedList;
use std::io::Cursor;
use quick_xml::events::*;
use quick_xml::Writer;

use utility::LinkUtil;

pub struct CoreXml<'a> {
  title: &'a str,
  subject: &'a str,
  creator: &'a str,
  keywords: &'a str,
  description: &'a str,
  last_modified_by: &'a str,
  revision: &'a str,
}

impl<'a> CoreXml<'a> {
  pub fn default() -> CoreXml<'a> {
    CoreXml {
      title: "",
      subject: "",
      creator: "",
      keywords: "",
      description: "",
      last_modified_by: "",
      revision: "",
    }
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = LinkedList::new();

    events
      .add_tag(b"dc:title", self.title)
      .add_tag(b"dc:subject", self.subject)
      .add_tag(b"dc:creator", self.creator)
      .add_tag(b"cp:keywords", self.keywords)
      .add_tag(b"dc:description", self.description)
      .add_tag(b"cp:lastModifiedBy", self.last_modified_by)
      .add_tag(b"cp:revision", self.revision)
      // TODO: <dcterms:created> and <dcterms:modified>
      .wrap_tag_with_attr(b"cp:coreProperties", vec![
        ("xmlns:cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"),
        ("xmlns:dc", "http://purl.org/dc/elements/1.1/"),
        ("xmlns:dcterms", "http://purl.org/dc/terms/"),
        ("xmlns:dcmitype", "http://purl.org/dc/dcmitype/"),
        ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
      ])
      .add_decl()
      .to_xml()
  }
}

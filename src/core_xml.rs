/// docProps/core.xml

use std::io::Cursor;
use quick_xml::events::*;
use quick_xml::Writer;

use utility::add_tag;

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
    let mut events = vec![
      Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))),
      Event::Start(BytesStart::borrowed(b"cp:coreProperties", b"cp:coreProperties".len())
        .with_attributes(vec![
          ("xmlns:cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"),
          ("xmlns:dc", "http://purl.org/dc/elements/1.1/"),
          ("xmlns:dcterms", "http://purl.org/dc/terms/"),
          ("xmlns:dcmitype", "http://purl.org/dc/dcmitype/"),
          ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
        ]))
    ];

    add_tag(&mut events, b"dc:title", self.title);
    add_tag(&mut events, b"dc:subject", self.subject);
    add_tag(&mut events, b"dc:creator", self.creator);
    add_tag(&mut events, b"cp:keywords", self.keywords);
    add_tag(&mut events, b"dc:description", self.description);
    add_tag(&mut events, b"cp:lastModifiedBy", self.last_modified_by);
    add_tag(&mut events, b"cp:revision", self.revision);
    // TODO: <dcterms:created> and <dcterms:modified>

    events.push(Event::End(BytesEnd::borrowed(b"cp:coreProperties")));

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    for event in events {
      writer.write_event(event);
    }

    writer.into_inner().into_inner()
  }
}

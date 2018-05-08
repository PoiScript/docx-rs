use quick_xml::events::Event;
use std::collections::LinkedList;

use element::Element;
use events_list::EventListExt;

static CORE_PROPERTIES_NAMESPACES: [(&'static str, &'static str); 5] = [
  (
    "xmlns:cp",
    "http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
  ),
  ("xmlns:dc", "http://purl.org/dc/elements/1.1/"),
  ("xmlns:dcterms", "http://purl.org/dc/terms/"),
  ("xmlns:dcmitype", "http://purl.org/dc/dcmitype/"),
  ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
];

pub struct CoreXml<'a> {
  title: &'a str,
  subject: &'a str,
  creator: &'a str,
  keywords: &'a str,
  description: &'a str,
  last_modified_by: &'a str,
  revision: &'a str,
}

impl<'a> Element<'a> for CoreXml<'a> {
  fn default() -> CoreXml<'a> {
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

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    events
      .add_text_tag("dc:title", self.title)
      .add_text_tag("dc:title", self.title)
      .add_text_tag("dc:subject", self.subject)
      .add_text_tag("dc:creator", self.creator)
      .add_text_tag("cp:keywords", self.keywords)
      .add_text_tag("dc:description", self.description)
      .add_text_tag("cp:lastModifiedBy", self.last_modified_by)
      .add_text_tag("cp:revision", self.revision)
      // TODO: <dcterms:created> and <dcterms:modified>
      .warp_attrs_tag("cp:coreProperties", CORE_PROPERTIES_NAMESPACES.to_vec());

    events
  }
}

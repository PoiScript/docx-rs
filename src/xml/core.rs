use quick_xml::events::Event;
use std::collections::LinkedList;

use events_list::EventListExt;
use schema::SCHEMA_CORE;
use xml::Xml;

static CORE_PROPERTIES_NAMESPACES: [(&str, &str); 1] = [("xmlns:cp", SCHEMA_CORE)];

#[derive(Debug)]
pub struct CoreXml<'a> {
  title: &'a str,
  subject: &'a str,
  creator: &'a str,
  keywords: &'a str,
  description: &'a str,
  last_modified_by: &'a str,
  revision: &'a str,
}

impl<'a> Xml<'a> for CoreXml<'a> {
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

use quick_xml::events::Event;
use std::collections::LinkedList;

use body::Para;
use element::Element;
use events_list::EventListExt;

static DOCUMENT_NAMESPACES: [(&'static str, &'static str); 9] = [
  (
    "xmlns:ve",
    "http://schemas.openxmlformats.org/markup-compatibility/2006",
  ),
  ("xmlns:o", "urn:schemas-microsoft-com:office:office"),
  (
    "xmlns:o12",
    "http://schemas.microsoft.com/office/2004/7/core",
  ),
  (
    "xmlns:r",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
  ),
  (
    "xmlns:m",
    "http://schemas.microsoft.com/office/omml/2004/12/core",
  ),
  ("xmlns:v", "urn:schemas-microsoft-com:vml"),
  (
    "xmlns:wp",
    "http://schemas.openxmlformats.org/drawingml/2006/3/wordprocessingDrawing",
  ),
  ("xmlns:w10", "urn:schemas-microsoft-com:office:word"),
  (
    "xmlns:w",
    "http://schemas.openxmlformats.org/wordprocessingml/2006/3/main",
  ),
];

pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>,
}

impl<'a> DocumentXml<'a> {
  pub fn add_para(&mut self, para: Para<'a>) {
    self.body.push(para);
  }
}

impl<'a> Element<'a> for DocumentXml<'a> {
  fn default() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    // TODO
    //    for i in 0..self.body.len() {
    //      for tag in self.body[i].tags() {
    //        tags.push(tag);
    //      }
    //    }

    events
      .warp_tag("w:body")
      .warp_attrs_tag("w:document", DOCUMENT_NAMESPACES.to_vec());

    events
  }
}

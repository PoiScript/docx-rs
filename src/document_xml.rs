use std::collections::LinkedList;
use quick_xml::events::*;

use body::Para;
use utility::LinkUtil;

pub struct DocumentXml<'a> {
  body: Vec<Para<'a>>
}

impl<'a> DocumentXml<'a> {
  pub fn new() -> DocumentXml<'a> {
    DocumentXml { body: Vec::new() }
  }

  pub fn xml_events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for i in 0..self.body.len() {
      for event in self.body[i].xml_events() {
        events.push_back(event);
      }
    }

    events
  }

  pub fn add_para(&mut self, para: Para<'a>) {
    self.body.push(para);
  }

  pub fn generate(&self) -> Vec<u8> {
    let mut events = self.xml_events();

    events
      .warp_tag(b"w:body")
      .wrap_tag_with_attr(b"w:document", vec![
        ("xmlns:ve", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
        ("xmlns:o", "urn:schemas-microsoft-com:office:office"),
        ("xmlns:o12", "http://schemas.microsoft.com/office/2004/7/core"),
        ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
        ("xmlns:m", "http://schemas.microsoft.com/office/omml/2004/12/core"),
        ("xmlns:v", "urn:schemas-microsoft-com:vml"),
        ("xmlns:wp", "http://schemas.openxmlformats.org/drawingml/2006/3/wordprocessingDrawing"),
        ("xmlns:w10", "urn:schemas-microsoft-com:office:word"),
        ("xmlns:w", "http://schemas.openxmlformats.org/wordprocessingml/2006/3/main"),
      ])
      .add_decl()
      .to_xml()
  }
}

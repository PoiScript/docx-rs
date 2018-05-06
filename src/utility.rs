// utility functions

use std::io::Cursor;
use quick_xml::events::{BytesStart, BytesEnd, BytesDecl, BytesText, Event};
use quick_xml::events::attributes::Attribute;
use quick_xml::Writer;
use std::collections::LinkedList;

pub trait LinkUtil<'a> {
  fn add_decl(&mut self) -> &mut Self;
  fn add_tag(&mut self, tag: &'a [u8], content: &'a str) -> &mut Self;
  fn add_tag_with_attr(&mut self, tag: &'a [u8], attributes: Vec<(&'a str, &'a str)>) -> &mut Self;
  fn warp_tag(&mut self, tag: &'a [u8]) -> &mut Self;
  fn wrap_tag_with_attr(&mut self, tag: &'a [u8], attributes: Vec<(&'a str, &'a str)>) -> &mut Self;
  fn to_xml(&self) -> Vec<u8>;
}

impl<'a> LinkUtil<'a> for LinkedList<Event<'a>> {
  fn add_decl(&mut self) -> &mut Self {
    self.push_front(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));

    self
  }

  fn add_tag(&mut self, tag: &'a [u8], content: &'a str) -> &mut Self {
    self.push_back(Event::Start(BytesStart::borrowed(tag, tag.len())));
    self.push_back(Event::Text(BytesText::from_plain_str(content)));
    self.push_back(Event::End(BytesEnd::borrowed(tag)));

    self
  }

  fn add_tag_with_attr(&mut self, tag: &'a [u8], attributes: Vec<(&'a str, &'a str)>) -> &mut Self {
    self.push_back(Event::Start(BytesStart::borrowed(tag, tag.len()).with_attributes(attributes)));
    self.push_back(Event::End(BytesEnd::borrowed(tag)));

    self
  }

  fn warp_tag(&mut self, tag: &'a [u8]) -> &mut Self {
    self.push_front(Event::Start(BytesStart::borrowed(tag, tag.len())));
    self.push_back(Event::End(BytesEnd::borrowed(tag)));

    self
  }

  fn wrap_tag_with_attr(&mut self, tag: &'a [u8], attributes: Vec<(&'a str, &'a str)>) -> &mut Self {
    self.push_front(Event::Start(BytesStart::borrowed(tag, tag.len()).with_attributes(attributes)));
    self.push_back(Event::End(BytesEnd::borrowed(tag)));

    self
  }

  fn to_xml(&self) -> Vec<u8> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    for event in self {
      writer.write_event(event);
    }

    writer.into_inner().into_inner()
  }
}

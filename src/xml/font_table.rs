use quick_xml::events::*;
use std::collections::LinkedList;

use events_list::EventListExt;
use schema::{SCHEMA_MAIN, SCHEMA_RELATIONSHIPS};
use xml::Xml;

#[derive(Debug)]
pub struct Font<'a> {
  name: &'a str,
  charset: &'a str,
  family: &'a str,
  pitch: &'a str,
}

impl<'a> Xml<'a> for Font<'a> {
  fn default() -> Font<'a> {
    Font {
      name: "Times New Roman",
      charset: "00",
      family: "roman",
      pitch: "variable",
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    events
      .add_attrs_empty_tag("w:charset", vec![("w:val", self.charset)])
      .add_attrs_empty_tag("w:family", vec![("w:val", self.family)])
      .add_attrs_empty_tag("w:pitch", vec![("w:val", self.pitch)])
      .warp_attrs_tag("w:font", vec![("w:name", self.name)]);

    events
  }
}

#[derive(Debug)]
pub struct FontTableXml<'a> {
  fonts: Vec<Font<'a>>,
}

impl<'a> Xml<'a> for FontTableXml<'a> {
  fn default() -> FontTableXml<'a> {
    FontTableXml {
      fonts: vec![Font::default()],
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for font in &self.fonts {
      events.append(&mut font.events());
    }

    events.warp_attrs_tag(
      "w:fonts",
      vec![("xmlns:w", SCHEMA_MAIN), ("xmlns:r", SCHEMA_RELATIONSHIPS)],
    );

    events
  }
}

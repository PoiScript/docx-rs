use std::collections::LinkedList;
use std::io::Cursor;
use quick_xml::events::*;
use quick_xml::Writer;

use utility::{add_tag, warp_tag};

// Specifies a run of content within the paragraph.
pub struct Run<'a> {
  text: &'a str,
  props: Vec<RunProp>,
}

impl<'a> Run<'a> {
  fn xml_events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    // TODO: run props
    add_tag(&mut events, b"w:t", self.text);
    warp_tag(&mut events, b"w:r");

    events
  }
}

// Specifies properties for the run.
pub enum RunProp {}

pub struct Para<'a> {
  props: Vec<ParaProp>,
  runs: Vec<Run<'a>>,
}

impl<'a> Para<'a> {
  pub fn xml_events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    // TODO: para props
    for i in 0..self.runs.len() {
      for event in self.runs[i].xml_events() {
        events.push_back(event);
      }
    }
    warp_tag(&mut events, b"w:p");

    events
  }
}

// Specifies a set of properties for the paragraph.
pub enum ParaProp {}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}

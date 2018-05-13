use quick_xml::events::Event;
use std::collections::LinkedList;
use std::default::Default;

use events_list::EventListExt;
use xml::Xml;

// Specifies a run of content within the paragraph.
#[derive(Debug)]
pub struct Run<'a> {
  text: &'a str,
  props: Vec<RunProp>,
}

impl<'a> Run<'a> {
  fn new(text: &'a str) -> Run<'a> {
    Run {
      text,
      props: Vec::new(),
    }
  }
}

impl<'a> Default for Run<'a> {
  fn default() -> Run<'a> {
    Run {
      text: "",
      props: Vec::new(),
    }
  }
}

impl<'a> Xml<'a> for Run<'a> {
  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    // TODO: run props
    events.add_text_tag("w:t", self.text).warp_tag("w:r");

    events
  }
}

// Specifies properties for the run.
#[derive(Debug)]
pub enum RunProp {}

#[derive(Debug)]
pub struct Para<'a> {
  props: Vec<ParaProp>,
  runs: Vec<Run<'a>>,
}

impl<'a> Para<'a> {
  pub fn new(text: &'a str) -> Para<'a> {
    Para {
      runs: vec![Run::new(text)],
      props: Vec::new(),
    }
  }
}

impl<'a> Default for Para<'a> {
  fn default() -> Para<'a> {
    Para {
      runs: Vec::new(),
      props: Vec::new(),
    }
  }
}

impl<'a> Xml<'a> for Para<'a> {
  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    for run in &self.runs {
      events.append(&mut run.events());
    }

    events.warp_tag("w:p");

    events
  }
}

// Specifies a set of properties for the paragraph.
#[derive(Debug)]
pub enum ParaProp {}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}

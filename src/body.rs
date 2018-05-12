use quick_xml::events::Event;
use std::collections::LinkedList;

use events_list::EventListExt;
use xml::Xml;

// Specifies a run of content within the paragraph.
pub struct Run<'a> {
  text: &'a str,
  props: Vec<RunProp>,
}

impl<'a> Xml<'a> for Run<'a> {
  fn default() -> Run<'a> {
    Run {
      text: "",
      props: Vec::new(),
    }
  }

  fn events(&self) -> LinkedList<Event<'a>> {
    let mut events = LinkedList::new();

    // TODO: run props
    events.add_text_tag("w:t", self.text).warp_tag("w:r");

    events
  }
}

// Specifies properties for the run.
pub enum RunProp {}

pub struct Para<'a> {
  props: Vec<ParaProp>,
  runs: Vec<Run<'a>>,
}

impl<'a> Xml<'a> for Para<'a> {
  fn default() -> Para<'a> {
    Para {
      runs: Vec::new(),
      props: Vec::new(),
    }
  }

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
pub enum ParaProp {}

// Specifies the contents of the body of the document.
pub enum _Content<'a> {
  Para(Para<'a>),
  Table,
  SecProp,
}

// utility functions

use std::io::Cursor;
use quick_xml::events::{BytesStart, BytesText, BytesEnd, Event};
use quick_xml::Writer;
use std::collections::LinkedList;

#[inline]
pub fn add_tag<'a>(events: &mut LinkedList<Event<'a>>, tag: &'a [u8], content: &'a str) {
  events.push_back(Event::Start(BytesStart::borrowed(tag, tag.len())));
  events.push_back(Event::Text(BytesText::from_plain_str(content)));
  events.push_back(Event::End(BytesEnd::borrowed(tag)));
}

#[inline]
pub fn events_to_xml<'a>(events: &LinkedList<Event<'a>>) -> Vec<u8> {
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  for event in events {
    writer.write_event(event);
  }

  writer.into_inner().into_inner()
}

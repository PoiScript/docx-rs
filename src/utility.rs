// utility functions

use quick_xml::events::{BytesStart, BytesText, BytesEnd, Event};

pub fn add_tag<'a>(events: &mut Vec<Event<'a>>, tag: &'a [u8], content: &'a str) {
  events.push(Event::Start(BytesStart::borrowed(tag, tag.len())));
  events.push(Event::Text(BytesText::from_plain_str(content)));
  events.push(Event::End(BytesEnd::borrowed(tag)));
}

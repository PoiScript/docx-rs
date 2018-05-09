use quick_xml::events::*;
use std::collections::LinkedList;
use std::iter::IntoIterator;

pub trait EventListExt<'a> {
  fn add_empty_tag(&'a mut self, name: &'a str) -> &mut Self;

  fn add_text_tag(&mut self, name: &'a str, content: &'a str) -> &mut Self;

  fn warp_tag(&mut self, name: &'a str) -> &mut Self;

  fn add_attrs_empty_tag<I>(&'a mut self, name: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>;

  fn add_attrs_text_tag<I>(&mut self, name: &'a str, content: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>;

  fn warp_attrs_tag<I>(&mut self, name: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>;
}

impl<'a> EventListExt<'a> for LinkedList<Event<'a>> {
  fn add_empty_tag(&'a mut self, name: &'a str) -> &mut Self {
    self.push_back(Event::Empty(BytesStart::borrowed(
      name.as_bytes(),
      name.len(),
    )));
    self
  }

  #[inline]
  fn add_text_tag(&mut self, name: &'a str, content: &'a str) -> &mut Self {
    self.push_back(Event::Start(BytesStart::borrowed(
      name.as_bytes(),
      name.len(),
    )));
    self.push_back(Event::Text(BytesText::from_plain(content.as_bytes())));
    self.push_back(Event::End(BytesEnd::borrowed(name.as_bytes())));
    self
  }

  #[inline]
  fn warp_tag(&mut self, name: &'a str) -> &mut Self {
    self.push_front(Event::Start(BytesStart::borrowed(
      name.as_bytes(),
      name.len(),
    )));
    self.push_back(Event::End(BytesEnd::borrowed(name.as_bytes())));
    self
  }

  #[inline]
  fn add_attrs_empty_tag<I>(&'a mut self, name: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>,
  {
    self.push_back(Event::Empty(
      BytesStart::borrowed(name.as_bytes(), name.len()).with_attributes(attrs),
    ));
    self
  }

  #[inline]
  fn add_attrs_text_tag<I>(&mut self, name: &'a str, content: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>,
  {
    self.push_back(Event::Start(
      BytesStart::borrowed(name.as_bytes(), name.len()).with_attributes(attrs),
    ));
    self.push_back(Event::Text(BytesText::from_plain(content.as_bytes())));
    self.push_back(Event::End(BytesEnd::borrowed(name.as_bytes())));
    self
  }

  #[inline]
  fn warp_attrs_tag<I>(&mut self, name: &'a str, attrs: I) -> &mut Self
  where
    I: IntoIterator<Item = (&'a str, &'a str)>,
  {
    self.push_front(Event::Start(
      BytesStart::borrowed(name.as_bytes(), name.len()).with_attributes(attrs),
    ));
    self.push_back(Event::End(BytesEnd::borrowed(name.as_bytes())));
    self
  }
}

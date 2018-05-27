macro_rules! attrs {
  ($start:tt, $key:expr, $value:expr) => {{
    $start.push_attribute(($key, $value));
  }};

  ($start:tt, $key:expr, $value:expr, $($rest:tt)*) => {{
    $start.push_attribute(($key, $value));
    attrs!($start, $($rest)*);
  }};
}

macro_rules! bytes_start {
  ($tag:tt) => {{
    BytesStart::borrowed($tag, $tag.len())
  }};

  ($tag:tt[$($attrs:tt)*]) => {{
    let mut start = BytesStart::borrowed($tag, $tag.len());
    attrs!(start, $($attrs)*);
    start
  }};
}

macro_rules! tag {
  ($writer:ident, $tag:tt) => {{
    $writer.write_event(Event::Empty(bytes_start!($tag)))?;
  }};

  ($writer:ident, $tag:tt[$($attrs:tt)*]) => {{
    $writer.write_event(Event::Empty(bytes_start!($tag[$($attrs)*])))?;
  }};

  ($writer:ident, $tag:tt { $block:block }) => {{
    $writer.write_event(Event::Start(bytes_start!($tag)))?;
    $block
    $writer.write_event(Event::End(BytesEnd::borrowed($tag)))?;
  }};

  ($writer:ident, $tag:tt[$($attrs:tt)*] { $block:block }) => {{
    $writer.write_event(Event::Start(bytes_start!($tag[$($attrs)*])))?;
    $block
    $writer.write_event(Event::End(BytesEnd::borrowed($tag)))?;
  }};

  ($writer:ident, $tag:tt { $text:expr }) => {{
    $writer.write_event(Event::Start(bytes_start!($tag)))?;
    $writer.write_event(Event::Text(BytesText::from_plain($text.as_bytes())))?;
    $writer.write_event(Event::End(BytesEnd::borrowed($tag)))?;
  }};

  ($writer:ident, $tag:tt[$($attrs:tt)*] { $text:expr }) => {{
    $writer.write_event(Event::Start(bytes_start!($tag[$($attrs)*])))?;
    $writer.write_event(Event::Text(BytesText::from_plain($text.as_bytes())))?;
    $writer.write_event(Event::End(BytesEnd::borrowed($tag)))?;
  }};
}

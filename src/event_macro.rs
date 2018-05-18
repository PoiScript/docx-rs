macro_rules! attrs {
  ($start:tt, $key:expr, $value:expr) => {{
    $start.push_attribute(($key, $value));
  }};

  ($start:tt, $key:expr, $value:expr, $($rest:tt)*) => {{
    $start.push_attribute(($key, $value));
    attrs!($start, $($rest)*);
  }};
}

macro_rules! write_empty_event {
  ($writer:ident, $tag:tt) => {{
    $writer.write_event(Event::Empty(BytesStart::borrowed($tag, $tag.len())))?;
  }};

  ($writer:ident, $tag:tt, $($attrs:tt)*) => {{
    let mut start = BytesStart::borrowed($tag, $tag.len());
    attrs!(start, $($attrs)*);
    $writer.write_event(Event::Empty(start))?;
  }};
}

macro_rules! write_start_event {
  ($writer:ident, $tag:tt) => {{
    $writer.write_event(Event::Start(BytesStart::borrowed($tag, $tag.len())))?;
  }};

  ($writer:ident, $tag:tt, $($attrs:tt)*) => {{
    let mut start = BytesStart::borrowed($tag, $tag.len());
    attrs!(start, $($attrs)*);
    $writer.write_event(Event::Start(start))?;
  }};
}

macro_rules! write_text_event {
  ($writer:ident, $tag:tt) => {{
    $writer.write_event(Event::Text(BytesText::from_plain($tag.as_bytes())))?;
  }};
}

macro_rules! write_end_event {
  ($writer:ident, $tag:tt) => {{
    $writer.write_event(Event::End(BytesEnd::borrowed($tag)))?;
  }};
}

macro_rules! write_events {
  ($writer:ident,) => (());

  ($writer:ident, ( $tag:tt, $($attrs:tt)* ) { $($inner:tt)* } $($rest:tt)*) => {{
    write_start_event!($writer, $tag, $($attrs)*);
    write_events!($writer, $($inner)*);
    write_end_event!($writer, $tag);
    write_events!($writer, $($rest)*);
  }};

  ($writer:ident, ( $tag:tt, $($attrs:tt)* ) $($rest:tt)*) => {{
    write_empty_event!($writer, $tag, $($attrs)*);
    write_events!($writer, $($rest)*);
  }};

  ($writer:ident, $tag:tt { $($inner:tt)* } $($rest:tt)*) => {{
    write_start_event!($writer, $tag);
    write_events!($writer, $($inner)*);
    write_end_event!($writer, $tag);
    write_events!($writer, $($rest)*);
  }};

  ($writer:ident, $tag:expr) => {{
    write_text_event!($writer, $tag);
  }};
}

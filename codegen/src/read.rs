use proc_macro2::TokenStream;
use types::{Enum, Struct};

pub(crate) fn impl_read_struct(s: &Struct) -> TokenStream {
  let match_read = match_read(&s);

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        #match_read
        Event::Eof => return Err(Error::UnexpectedEof),
        _ => (),
      };
      buf.clear();
    }
  }
}

fn match_read(s: &Struct) -> TokenStream {
  let name = &s.name;
  let tag = &s.attrs.value;

  if s.attrs.key == "parent" || s.attrs.key == "text" {
    quote! {
      Event::Start(ref e) => {
        if e.name() == #tag.as_bytes() {
            return #name::read_with_bytes_start(e, r);
          } else {
            return Err(Error::UnexpectedTag {
              expected: String::from(#tag),
              found: String::from_utf8(e.name().to_vec())?,
            });
          }
        },
      Event::Empty(_) => {
        return Err(Error::UnexpectedEvent {
          expected: String::from("Start"),
          found: String::from("Empty"),
        });
      },
      Event::End(_) => {
        return Err(Error::UnexpectedEvent {
          expected: String::from("Start"),
          found: String::from("End"),
        });
      }
    }
  } else if s.attrs.key == "empty" {
    quote! {
      Event::Empty(ref e) => {
        if e.name() == #tag.as_bytes() {
            return #name::read_with_bytes_start(e, r);
          } else {
            return Err(Error::UnexpectedTag {
              expected: String::from(#tag),
              found: String::from_utf8(e.name().to_vec())?,
            });
          }
        },
      Event::Start(_) => {
        return Err(Error::UnexpectedEvent {
          expected: String::from("Empty"),
          found: String::from("Start"),
        });
      },
      Event::End(_) => {
        return Err(Error::UnexpectedEvent {
          expected: String::from("Empty"),
          found: String::from("End"),
        });
      }
    }
  } else {
    unreachable!();
  }
}

pub(crate) fn impl_read_enum(e: &Enum) -> TokenStream {
  let start_tags: &Vec<_> = &e
    .texts_and_children()
    .iter()
    .map(|f| &f.attrs.value as &str)
    .collect();

  let start_tag_names = start_tags.join(", ");

  let empty_tags: &Vec<_> = &e.empties().iter().map(|f| &f.attrs.value as &str).collect();

  let empty_tag_names = empty_tags.join(", ");

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        Event::Start(ref e) => {
          let tag = std::str::from_utf8(e.name())?;
          match tag {
            #( #start_tags => return Self::read_with_bytes_start(e, r), )*
            _ => return Err(Error::UnexpectedTag {
              expected: String::from(#start_tag_names),
              found: String::from(tag),
            }),
          }
        },
        Event::Empty(ref e) => {
          let tag = std::str::from_utf8(e.name())?;
          match tag {
            #( #empty_tags => return Self::read_with_bytes_start(e, r), )*
            _ => return Err(Error::UnexpectedTag {
              expected: String::from(#empty_tag_names),
              found: String::from(tag),
            }),
          }
        },
        Event::Eof => return Err(Error::UnexpectedEof),
        _ => (),
      }
    }
  }
}

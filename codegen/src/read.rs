use proc_macro2::TokenStream;
use types::{Enum, Struct};

pub(crate) fn impl_read_struct(s: &Struct) -> TokenStream {
  let match_read = match_read(&s);

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        #match_read
        Event::Eof | Event::End(_) => break,
        _ => (),
      };
      buf.clear();
    }
    unreachable!();
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
          expected: String::from("Start"),
          found: String::from("Empty"),
        });
      },
    }
  } else {
    unreachable!();
  }
}

pub(crate) fn impl_read_enum(e: &Enum) -> TokenStream {
  let text_tags: &Vec<_> = &e
    .filter_field("text")
    .iter()
    .map(|f| &f.attrs.value)
    .collect();

  let child_tags: &Vec<_> = &e
    .filter_field("child")
    .iter()
    .map(|f| &f.attrs.value)
    .collect();

  let empty_tags: &Vec<_> = &e
    .filter_field("empty")
    .iter()
    .map(|f| &f.attrs.value)
    .collect();

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        Event::Start(ref e) => {
          match std::str::from_utf8(e.name())? {
            #( #text_tags => return Self::read_with_bytes_start(e, r), )*
            #( #child_tags => return Self::read_with_bytes_start(e, r), )*
            _ => unreachable!(), // TODO: throws an error
          }
        },
        Event::Empty(ref e) => {
          match std::str::from_utf8(e.name())? {
            #( #empty_tags => return Self::read_with_bytes_start(e, r), )*
            _ => unreachable!(), // TODO: throws an error
          }
        },
        _ => (),
      }
    }
  }
}

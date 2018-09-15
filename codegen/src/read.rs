use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use types::{Enum, Struct};

pub(crate) fn impl_read_struct(s: &Struct) -> TokenStream {
  let match_read = match_read(&s);

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf) {
        #match_read
        Ok(Event::Eof) | Ok(Event::End(_)) => break,
        _ => (),
      };
      buf.clear();
    }
    unreachable!();
  }
}

fn match_read(s: &Struct) -> TokenStream {
  let name = Ident::new(&s.name, Span::call_site());
  let tag = &s.attrs.value;

  if s.attrs.key == "parent" || s.attrs.key == "text" {
    quote! {
      Ok(Event::Start(ref e)) => {
        if e.name() == #tag.as_bytes() {
            return #name::read_with_bytes_start(e, r);
          } else {
            // TODO: throws an error
          }
        },
      Ok(Event::Empty(_)) => {
        // TODO: throws an error
      },
    }
  } else if s.attrs.key == "empty" {
    quote! {
      Ok(Event::Empty(ref e)) => {
        if e.name() == #tag.as_bytes() {
            return #name::read_with_bytes_start(e, r);
          } else {
            // TODO: throws an error
          }
        },
      Ok(Event::Start(_)) => {
        // TODO: throws an error
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
      match r.read_event(&mut buf) {
        Ok(Event::Start(ref e)) => {
          match std::str::from_utf8(e.name()).unwrap() {
            #( #text_tags => return Self::read_with_bytes_start(e, r), )*
            #( #child_tags => return Self::read_with_bytes_start(e, r), )*
            _ => unreachable!(), // TODO: throws an error
          }
        },
        Ok(Event::Empty(ref e)) => {
          match std::str::from_utf8(e.name()).unwrap() {
            #( #empty_tags => return Self::read_with_bytes_start(e, r), )*
            _ => unreachable!(), // TODO: throws an error
          }
        },
        _ => unreachable!(), // TODO: throws an error
      }
    }
  }
}

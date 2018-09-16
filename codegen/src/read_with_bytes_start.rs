use proc_macro2::TokenStream;
use types::{Enum, Struct};

pub(crate) fn impl_read_with_bytes_start_struct(s: &Struct) -> TokenStream {
  let name = &s.name;

  let init = initialize_value(&s);
  let match_attr = match_attr(&s);
  let match_value = match_value(&s);
  let return_value = return_value(&s);

  quote! {
    #init

    for attr in bs.attributes().filter_map(|a| a.ok()) {
      match std::str::from_utf8(attr.key)? {
        #match_attr
        _ => (),
      }
    }

    #match_value

    Ok(#name { #( #return_value, )* })
  }
}

fn initialize_value(s: &Struct) -> TokenStream {
  let names: &Vec<_> = &s.attrs().iter().map(|f| &f.name).collect();

  if s.attrs.key == "parent" {
    let init_children: &Vec<_> = &s
      .texts_and_children()
      .iter()
      .map(|f| {
        let name = &f.name;
        if f.is_vec {
          quote!{ let mut #name = Vec::new(); }
        } else {
          quote!{ let mut #name = None; }
        }
      }).collect();

    quote! {
      #( let mut #names = None; )*
      #( #init_children )*
    }
  } else if s.attrs.key == "text" {
    quote! {
      #( let mut #names = None; )*
      let text = r.read_text(bs.name(), &mut Vec::new())?;
    }
  } else {
    quote! {
      #( let mut #names = None; )*
    }
  }
}

fn match_attr(s: &Struct) -> TokenStream {
  let names: &Vec<_> = &s.attrs().iter().map(|f| &f.name).collect();

  let tags: &Vec<_> = &s.attrs().iter().map(|f| &f.attrs.value).collect();

  quote! {
    #( #tags => #names = Some(String::from_utf8(attr.value.into_owned().to_vec())?), )*
  }
}

fn match_value(s: &Struct) -> TokenStream {
  if s.attrs.key == "parent" {
    let children_matches: &Vec<_> = &s
      .children()
      .iter()
      .map(|f| {
        let tag = &f.attrs.value;
        let name = &f.name;
        let ty = &f.ty;

        if f.is_vec {
          quote! {
            #tag => #name.push(#ty::read_with_bytes_start(e, r)?),
          }
        } else {
          quote! {
            #tag => #name = Some(#ty::read_with_bytes_start(e, r)?),
          }
        }
      }).collect();

    let text_names: &Vec<_> = &s.texts().iter().map(|f| &f.name).collect();

    let text_tags: &Vec<_> = &s.texts().iter().map(|f| &f.attrs.value).collect();
    let text_tags1 = text_tags.clone();

    quote! {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::Start(ref e) | Event::Empty(ref e) => {
            match std::str::from_utf8(e.name())? {
              #( #children_matches )*
              #( #text_tags => #text_names = Some(r.read_text(#text_tags1, &mut Vec::new())?), )*
              _ => (),
            }
          },
          Event::End(_) => break,
          Event::Eof => return Err(Error::UnexpectedEof),
          _ => (),
        };
        buf.clear();
      }
    }
  } else {
    quote!()
  }
}

fn return_value(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.attrs() {
    let name = &f.name;
    result.push(if f.is_option {
      quote!{ #name }
    } else {
      quote!{ #name: #name.expect("bla") }
    })
  }

  if s.attrs.key == "parent" {
    for f in &s.texts_and_children() {
      let name = &f.name;
      result.push(if f.is_option || f.is_vec {
        quote!{ #name }
      } else {
        quote!{ #name: #name.expect("bla") }
      })
    }
  } else if s.attrs.key == "text" {
    let name = &s.texts().first().unwrap().name;
    result.push(quote! { #name: text });
  }

  result
}

pub(crate) fn impl_read_with_bytes_start_enum(e: &Enum) -> TokenStream {
  use std::iter::repeat;

  let names: &Vec<_> = &e.fields.iter().map(|f| &f.name).collect();

  let tags: &Vec<_> = &e.fields.iter().map(|f| &f.attrs.value as &str).collect();

  let tag_names = tags.join(", ");

  let types: &Vec<_> = &e.fields.iter().map(|f| &f.ty).collect();

  let e_names = repeat(&e.name);

  quote! {
    let tag = std::str::from_utf8(bs.name())?;
    match tag {
      #( #tags => Ok(#e_names::#names(#types::read_with_bytes_start(bs, r)?)), )*
      _ => Err(Error::UnexpectedTag {
        expected: String::from(#tag_names),
        found: String::from(tag),
      })
    }
  }
}

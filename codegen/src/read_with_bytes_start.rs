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

    Ok(#name {
      #return_value
    })
  }
}

fn initialize_value(s: &Struct) -> TokenStream {
  let names: &Vec<_> = &s.filter_field("attr").iter().map(|f| &f.name).collect();

  if s.attrs.key == "parent" {
    let init_children: &Vec<_> = &s
      .filter_field("child")
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
  let names: &Vec<_> = &s.filter_field("attr").iter().map(|f| &f.name).collect();

  let tags: &Vec<_> = &s
    .filter_field("attr")
    .iter()
    .map(|f| &f.attrs.value)
    .collect();

  quote! {
    #( #tags => #names = Some(String::from_utf8(attr.value.into_owned().to_vec())?), )*
  }
}

fn match_value(s: &Struct) -> TokenStream {
  if s.attrs.key == "parent" {
    let matches: &Vec<_> = &s
      .filter_field("child")
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

    quote! {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::Start(ref e) | Event::Empty(ref e) => {
            match std::str::from_utf8(e.name())? {
              #( #matches )*
              _ => (),
            }
          },
          Event::End(_) => break,
          Event::Eof => break,
          _ => (),
        };
        buf.clear();
      }
    }
  } else {
    quote!()
  }
}

fn return_value(s: &Struct) -> TokenStream {
  let attrs: &Vec<_> = &s
    .filter_field("attr")
    .iter()
    .map(|f| {
      let name = &f.name;
      if f.is_option {
        quote!{ #name, }
      } else {
        quote!{ #name: #name.expect("bla"), }
      }
    }).collect();

  if s.attrs.key == "parent" {
    let children: &Vec<_> = &s
      .filter_field("child")
      .iter()
      .map(|f| {
        let name = &f.name;
        if f.is_option || f.is_vec {
          quote!{ #name, }
        } else {
          quote!{ #name: #name.expect("bla"), }
        }
      }).collect();
    quote! {
      #( #attrs )*
      #( #children )*
    }
  } else if s.attrs.key == "text" {
    let name = &s.find_field("text").name;
    quote! {
      #( #attrs )*
      #name: text,
    }
  } else {
    quote! {
      #( #attrs )*
    }
  }
}

pub(crate) fn impl_read_with_bytes_start_enum(e: &Enum) -> TokenStream {
  use std::iter::repeat;

  let names: &Vec<_> = &e.fields.iter().map(|f| &f.name).collect();

  let tags: &Vec<_> = &e.fields.iter().map(|f| &f.attrs.value as &str).collect();

  let tag_names = tags.join(", ");

  let types: &Vec<_> = &e.fields.iter().map(|f| &f.ty).collect();

  let e_names = repeat(&e.name);

  quote! {
    match std::str::from_utf8(bs.name())? {
      #( #tags => Ok(#e_names::#names(#types::read_with_bytes_start(bs, r)?)), )*
      _ => Err(Error::UnexpectedTag {
        expected: String::from(#tag_names),
        found: String::from(std::str::from_utf8(bs.name())?),
      })
    }
  }
}

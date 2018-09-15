use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use std::iter;
use types::{Enum, Struct};

pub(crate) fn impl_read_with_bytes_start_struct(s: &Struct) -> TokenStream {
  let name = Ident::new(&s.name, Span::call_site());

  let init = initialize_value(&s);
  let match_attr = match_attr(&s);
  let match_value = match_value(&s);
  let return_value = return_value(&s);

  quote! {
    #init

    for attr in bs.attributes().filter_map(|a| a.ok()) {
      match std::str::from_utf8(attr.key).unwrap() {
        #match_attr
        _ => (),
      }
    }

    #match_value

    #name {
      #return_value
    }
  }
}

fn initialize_value(s: &Struct) -> TokenStream {
  let names: &Vec<_> = &s
    .filter_field("attr")
    .iter()
    .map(|f| Ident::new(&f.name, Span::call_site()))
    .collect();

  if s.attrs.key == "parent" {
    let init_children: &Vec<_> = &s
      .filter_field("child")
      .iter()
      .map(|f| {
        let name = Ident::new(&f.name, Span::call_site());
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
      let mut text = None;
    }
  } else {
    quote! {
      #( let mut #names = None; )*
    }
  }
}

fn match_attr(s: &Struct) -> TokenStream {
  let names: &Vec<_> = &s
    .filter_field("attr")
    .iter()
    .map(|f| Ident::new(&f.name, Span::call_site()))
    .collect();

  let tags: &Vec<_> = &s
    .filter_field("attr")
    .iter()
    .map(|f| &f.attrs.value)
    .collect();

  quote! {
    #( #tags => #names = Some(String::from_utf8(attr.value.into_owned().to_vec()).unwrap()), )*
  }
}

fn match_value(s: &Struct) -> TokenStream {
  if s.attrs.key == "parent" {
    let matches: &Vec<_> = &s
      .filter_field("child")
      .iter()
      .map(|f| {
        let tag = &f.attrs.value;
        let name = Ident::new(&f.name, Span::call_site());
        let ty = Ident::new(&f.ty, Span::call_site());

        if f.is_vec {
          quote! {
            #tag => #name.push(#ty::read_with_bytes_start(e, r)),
          }
        } else {
          quote! {
            #tag => #name = Some(#ty::read_with_bytes_start(e, r)),
          }
        }
      }).collect();

    quote! {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf) {
          Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
            match std::str::from_utf8(e.name()).unwrap() {
              #( #matches )*
              _ => (),
            }
          },
          Ok(Event::End(_)) => break,
          Ok(Event::Eof) => break,
          _ => (),
        };
        buf.clear();
      }
    }
  } else if s.attrs.key == "text" {
    quote!{
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf) {
          Ok(Event::Text(e)) => {
            text = Some(String::from_utf8(e.escaped().to_vec()).unwrap());
          },
          Ok(Event::End(_)) => break,
          Ok(Event::Eof) => break,
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
      let name = Ident::new(&f.name, Span::call_site());
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
        let name = Ident::new(&f.name, Span::call_site());
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
    let name = Ident::new(&s.find_field("text").name, Span::call_site());
    quote! {
      #( #attrs )*
      #name: text.expect("bla"),
    }
  } else {
    quote! {
      #( #attrs )*
    }
  }
}

pub(crate) fn impl_read_with_bytes_start_enum(e: &Enum) -> TokenStream {
  let names: &Vec<_> = &e
    .fields
    .iter()
    .map(|f| Ident::new(&f.name, Span::call_site()))
    .collect();

  let tags: &Vec<_> = &e.fields.iter().map(|f| &f.attrs.value).collect();

  let types: &Vec<_> = &e
    .fields
    .iter()
    .map(|f| Ident::new(&f.ty, Span::call_site()))
    .collect();

  let e_names: Vec<_> = iter::repeat(Ident::new(&e.name, Span::call_site()))
    .take(tags.len())
    .collect();

  quote! {
    match std::str::from_utf8(bs.name()).unwrap() {
      #( #tags => #e_names::#names(#types::read_with_bytes_start(bs, r)), )*
      _ => panic!("bla")  // TODO throws an error
    }
  }
}

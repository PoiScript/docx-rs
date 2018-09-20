use proc_macro2::TokenStream;
use syn::Ident;
use types::{Event, Item, ItemEnum, ItemStruct, Variant};

pub(crate) fn impl_read(item: &Item) -> TokenStream {
  match item {
    Item::Enum(e) => read_enum(e),
    Item::Struct(s) => read_struct(s),
  }
}

fn read_struct(s: &ItemStruct) -> TokenStream {
  let tag = &s.config.tag;
  let name = &s.name;

  let init_fields = init_fields(&s);
  let set_attrs = set_attrs(&s);
  let set_text = set_text(&s);
  let set_children = set_children(&s);
  let return_struct = return_struct(&s);

  match s.config.event {
    Event::Start => {
      quote! {
        #( #init_fields )*

        if let Some(bs) = bs {
          #set_attrs
        } else {
          let mut buf = Vec::new();
          loop {
            match r.read_event(&mut buf)? {
              Event::Start(ref bs) => {
                if bs.name() == #tag {
                  #set_attrs
                  break;
                } else {
                  return Err(Error::UnexpectedTag {
                    expected: String::from(stringify!(#tag)),
                    found: String::from_utf8(bs.name().to_vec())?,
                  });
                }
              },
              Event::Empty(_) => {
                return Err(Error::UnexpectedEvent {
                  expected: String::from("Empty"),
                  found: String::from("Start"),
                });
              },
              Event::Eof => return Err(Error::UnexpectedEof),
              _ => (),
            }
            buf.clear();
          }
        }

        #set_text

        #set_children

        Ok(#name { #( #return_struct )* })
      }
    }
    Event::Empty => {
      quote! {
        #( #init_fields )*

        if let Some(bs) = bs {
          #set_attrs
        } else {
          let mut buf = Vec::new();
          loop {
            match r.read_event(&mut buf)? {
              Event::Empty(ref bs) => {
                if bs.name() == #tag {
                  #set_attrs
                  break;
                } else {
                  return Err(Error::UnexpectedTag {
                    expected: String::from(stringify!(#tag)),
                    found: String::from_utf8(bs.name().to_vec())?,
                  });
                }
              },
              Event::Start(_) => {
                return Err(Error::UnexpectedEvent {
                  expected: String::from("Empty"),
                  found: String::from("Start"),
                });
              },
              Event::Eof => return Err(Error::UnexpectedEof),
              _ => (),
            }
            buf.clear();
          }
        }

        Ok(#name { #( #return_struct )* })
      }
    }
  }
}

fn init_fields(s: &ItemStruct) -> Vec<TokenStream> {
  s.fields
    .iter()
    .map(|f| {
      let name = &f.name;
      if f.is_vec().is_some() {
        quote! { let mut #name = Vec::new(); }
      } else {
        quote! { let mut #name = None; }
      }
    }).collect()
}

fn set_attrs(s: &ItemStruct) -> TokenStream {
  let match_attrs = s.fields.iter().filter(|f| f.config.is_attr).map(|f| {
    let name = &f.name.clone().unwrap();
    let tag = &f.config.attr.clone().unwrap();

    quote! { #tag => #name = Some(String::from_utf8(attr.value.into_owned().to_vec())?), }
  });

  quote! {
    for attr in bs.attributes().filter_map(|a| a.ok()) {
      match std::str::from_utf8(attr.key)? {
        #( #match_attrs )*
        _ => (),
      }
    }
  }
}

fn set_children(s: &ItemStruct) -> TokenStream {
  let match_children: &Vec<_> = &s
    .fields
    .iter()
    .filter(|f| f.config.is_child)
    .map(|f| {
      let tag = &f.config.tag;
      let name = f.name.clone().unwrap();
      let ty = &f.ty;

      if let Some(ty) = f.is_vec() {
        quote! {
          #tag => #name.push(#ty::read(r, Some(bs))?),
        }
      } else if let Some(ty) = f.is_option() {
        quote! {
          #tag => #name = Some(#ty::read(r, Some(bs))?),
        }
      } else {
        quote! {
          #tag => #name = Some(#ty::read(r, Some(bs))?),
        }
      }
    }).collect();

  let match_flattern_text: &Vec<_> = &s
    .fields
    .iter()
    .filter(|f| f.config.is_flattern_text)
    .map(|f| {
      let tag = &f.config.tag;
      let name = f.name.clone().unwrap();
      let tag1 = tag.clone();
      quote! { #tag => #name = Some(r.read_text(#tag1, &mut Vec::new())?), }
    }).collect();

  if match_flattern_text.len() == 0 && match_children.len() == 0 {
    return quote!();
  }

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        Event::Start(ref bs) | Event::Empty(ref bs) => {
          match bs.name() {
            #( #match_children )*
            #( #match_flattern_text )*
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
}

fn set_text(s: &ItemStruct) -> TokenStream {
  let field = match s.fields.iter().find(|f| f.config.is_text) {
    Some(f) => f,
    None => return quote!(),
  };
  let name = &field.name;
  let tag = &s.config.tag;

  quote! {
    #name = Some(r.read_text(#tag, &mut Vec::new())?);
  }
}

fn return_struct(s: &ItemStruct) -> Vec<TokenStream> {
  let struct_name = &s.name;
  s.fields
    .iter()
    .map(|f| {
      let name = &f.name;
      if f.is_option().is_some() || f.is_vec().is_some() {
        quote! { #name, }
      } else if f.is_cow_str() {
        quote! { #name : Cow::Owned(#name.ok_or(Error::MissingField {
          name: String::from(stringify!(#struct_name)),
          field: String::from(stringify!(#name)),
        })?), }
      } else {
        quote! { #name : #name.ok_or(Error::MissingField {
          name: String::from(stringify!(#struct_name)),
          field: String::from(stringify!(#name)),
        })?, }
      }
    }).collect()
}

fn read_enum(e: &ItemEnum) -> TokenStream {
  let name = &e.name;

  let match_start_variants: &Vec<_> = &e
    .variants
    .iter()
    .filter(|v| v.config.event == Event::Start)
    .map(|v| match_variant(v, name))
    .collect();

  let match_empty_variants: &Vec<_> = &e
    .variants
    .iter()
    .filter(|v| v.config.event == Event::Empty)
    .map(|v| match_variant(v, name))
    .collect();

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        Event::Start(ref bs) => {
          match bs.name() {
            #( #match_start_variants )*
            _ => return Err(Error::UnexpectedTag {
              expected: String::from(""),
              found: String::from(""),
            }),
          }
        },
        Event::Empty(ref bs) => {
          match bs.name() {
            #( #match_empty_variants )*
            _ => return Err(Error::UnexpectedTag {
              expected: String::from(""),
              found: String::from(""),
            }),
          }
        },
        Event::Eof => return Err(Error::UnexpectedEof),
        _ => (),
      }
    }
    buf.clear();
  }
}

fn match_variant(v: &Variant, enum_name: &Ident) -> TokenStream {
  let tag = &v.config.tag;
  let name = &v.name;
  let ty = &v.field.ty;

  quote!{ #tag => return #ty::read(r, Some(bs)).map(|p| #enum_name::#name(p)), }
}

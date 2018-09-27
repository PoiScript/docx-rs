use proc_macro2::{Span, TokenStream};
use syn::Ident;
use types::{Enum, Event, Item, Struct, TypeExt};

macro_rules! bytes_str {
  ($t:expr) => {
    ::syn::LitByteStr::new($t.value().as_bytes(), ::proc_macro2::Span::call_site())
  };
}

pub(crate) fn impl_read(item: &Item) -> TokenStream {
  match item {
    Item::Enum(e) => read_enum(e),
    Item::Struct(s) => read_struct(s),
  }
}

fn read_struct(s: &Struct) -> TokenStream {
  let name = &s.name;

  let init_fields = init_fields(&s);
  let set_attrs = set_attrs(&s);
  let set_text = set_text(&s);
  let set_children = set_children(&s);
  let return_struct = return_struct(&s);

  match s.event {
    Event::Start => {
      quote! {
        #init_fields

        #set_attrs

        #set_text

        #set_children

        Ok(#name { #return_struct })
      }
    }
    Event::Empty => {
      quote! {
        #init_fields

        #set_attrs

        Ok(#name { #return_struct })
      }
    }
  }
}

fn init_fields(s: &Struct) -> TokenStream {
  macro_rules! init_fld {
    ($t:tt) => {
      s.$t
        .iter()
        .map(|f| {
          let name = &f.name;
          if f.ty.is_vec().is_some() {
            quote! { let mut #name = Vec::new(); }
          } else if f.ty.is_bool() {
            quote! { let mut #name = false; }
          } else {
            quote! { let mut #name = None; }
          }
        }).collect::<Vec<_>>()
    };
  }

  let init_attr_flds = init_fld!(attr_flds);
  let init_child_flds = init_fld!(child_flds);
  let init_text_fld = init_fld!(text_fld);
  let init_flat_empty_flds = init_fld!(flat_empty_flds);
  let init_flat_empty_attr_flds = init_fld!(flat_empty_attr_flds);
  let init_flat_text_flds = init_fld!(flat_text_flds);

  quote! {
    #( #init_attr_flds )*
    #( #init_child_flds )*
    #( #init_text_fld )*
    #( #init_flat_empty_flds )*
    #( #init_flat_empty_attr_flds )*
    #( #init_flat_text_flds )*
  }
}

fn set_attrs(s: &Struct) -> TokenStream {
  let name = &s.name;

  let match_attrs: Vec<_> = s
    .attr_flds
    .iter()
    .map(|f| {
      let name = &f.name;
      let tag = bytes_str!(f.attr);
      let mut ty = &f.ty;

      if let Some(inner) = ty.is_option() {
        ty = inner;
      }

      if ty.is_string() {
        quote! { #tag => #name = Some(String::from_utf8(attr.value.into_owned().to_vec())?), }
      } else if ty.is_cow_str() {
        quote! { #tag => #name = Some(Cow::Owned(String::from_utf8(attr.value.into_owned().to_vec())?)), }
      }  else if ty.is_bool() {
        quote! {
          #tag => {
            let value = ::std::str::from_utf8(attr.value.borrow())?;
            #name = Some(bool::from_str(value).or(usize::from_str(value).map(|v| v != 0))?);
          }
        }
      } else {
        let ty = ty.get_ident();
        quote! { #tag => #name = Some(#ty::from_str(::std::str::from_utf8(attr.value.borrow())?)?), }
      }
    }).collect();

  if match_attrs.is_empty() {
    return quote!();
  }

  let tag = bytes_str!(s.tag);

  let loop_attrs = quote! {
    for attr in bs.attributes().filter_map(|a| a.ok()) {
      match attr.key {
        #( #match_attrs )*
        k => info!(
          "Unhandled attribute {} when parsing {}.",
          String::from_utf8_lossy(k),
          stringify!(#name)
        ),
      }
    }
  };

  let (event1, event2) = match s.event {
    Event::Start => (
      Ident::new("Start", Span::call_site()),
      Ident::new("Empty", Span::call_site()),
    ),
    Event::Empty => (
      Ident::new("Empty", Span::call_site()),
      Ident::new("Start", Span::call_site()),
    ),
  };

  quote! {
    if let Some(bs) = bs {
      #loop_attrs
    } else {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::#event1(ref bs) => {
            if bs.name() == #tag {
              #loop_attrs
              break;
            } else {
              return Err(Error::UnexpectedTag {
                expected: String::from(stringify!(#tag)),
                found: String::from_utf8(bs.name().to_vec())?,
              });
            }
          },
          Event::#event2(_) => {
            return Err(Error::UnexpectedEvent {
              expected: String::from(stringify!(#event1)),
              found: String::from(stringify!(#event2)),
            });
          },
          Event::Eof => return Err(Error::UnexpectedEof),
          _ => (),
        }
        buf.clear();
      }
    }
  }
}

fn set_children(s: &Struct) -> TokenStream {
  let tag = bytes_str!(s.tag);
  let name = &s.name;
  let match_children: &Vec<_> = &s
    .child_flds
    .iter()
    .map(|f| {
      let tags = f.tags.iter().map(|t| bytes_str!(t));
      let name = &f.name;

      if let Some(ty) = f.ty.is_vec() {
        let ident = ty.get_ident();
        quote! {
          #( #tags )|* => #name.push(#ident::read(r, Some(bs))?),
        }
      } else if let Some(ty) = f.ty.is_option() {
        let ident = ty.get_ident();
        quote! {
          #( #tags )|* => #name = Some(#ident::read(r, Some(bs))?),
        }
      } else {
        let ident = &f.ty.get_ident();
        quote! {
          #( #tags )|* => #name = Some(#ident::read(r, Some(bs))?),
        }
      }
    })
    .collect();

  let match_flatten_text: &Vec<_> = &s
    .flat_text_flds
    .iter()
    .map(|f| {
      let tag = bytes_str!(f.tag);
      let name = &f.name;
      let mut ty = &f.ty;
      if let Some(inner) = f.ty.is_option() {
        ty = inner;
      }
      if ty.is_cow_str() {
        quote! { #tag => #name = Some(Cow::Owned(r.read_text(#tag, &mut Vec::new())?)), }
      } else {
        quote! { #tag => #name = Some(r.read_text(#tag, &mut Vec::new())?), }
      }
    })
    .collect();

  let match_flatten_empty: &Vec<_> = &s
    .flat_empty_flds
    .iter()
    .map(|f| {
      let tag = bytes_str!(f.tag);
      let name = &f.name;
      quote! { #tag => #name = true, }
    })
    .collect();

  let match_flatten_empty_attr: &Vec<_> = &s
    .flat_empty_attr_flds
    .iter()
    .map(|f| {
      let tag = bytes_str!(f.tag);
      let name = &f.name;
      let key = bytes_str!(f.attr);
      let mut ty = &f.ty;

      if let Some(inner) = ty.is_option() {
        ty = inner;
      }

      let value = if ty.is_string() {
        quote! { String::from_utf8(attr.value.into_owned().to_vec())? }
      } else if ty.is_cow_str() {
        quote! { Cow::Owned(String::from_utf8(attr.value.into_owned().to_vec())?) }
      } else if ty.is_bool() {
        quote! {{
          let value = ::std::str::from_utf8(attr.value.borrow())?;
          bool::from_str(value).or(usize::from_str(value).map(|v| v != 0))?
        }}
      } else {
        let ty = ty.get_ident();
        quote! { #ty::from_str(::std::str::from_utf8(attr.value.borrow())?)? }
      };

      let default_value = if ty.is_bool() {
        quote!{ #name = Some(true); }
      } else {
        quote!()
      };

      quote! {
        #tag => {
          #default_value
          for attr in bs.attributes().filter_map(|a| a.ok()) {
            match attr.key {
              #key => #name = Some(#value),
              k => info!(
                "Unhandled attribute {} when parsing {}.",
                String::from_utf8_lossy(k),
                stringify!(#name)
              ),
            }
          }
        }
      }
    }).collect();

  if match_flatten_text.is_empty()
    && match_children.is_empty()
    && match_flatten_empty.is_empty()
    && match_flatten_empty_attr.is_empty()
  {
    return quote!();
  }

  quote! {
    let mut buf = Vec::new();
    loop {
      match r.read_event(&mut buf)? {
        Event::Start(ref bs) | Event::Empty(ref bs) => {
          match bs.name() {
            #( #match_children )*
            #( #match_flatten_text )*
            #( #match_flatten_empty )*
            #( #match_flatten_empty_attr )*
            t => info!(
              "Unhandled tag {} when parsing {}.",
              String::from_utf8_lossy(t),
              stringify!(#name)
            ),
          }
        },
        Event::End(ref bs) => if bs.name() == #tag { break; },
        Event::Eof => return Err(Error::UnexpectedEof),
        _ => (),
      };
      buf.clear();
    }
  }
}

fn set_text(s: &Struct) -> TokenStream {
  let field = match &s.text_fld {
    Some(f) => f,
    None => return quote!(),
  };
  let name = &field.name;
  let tag = bytes_str!(s.tag);

  if field.ty.is_cow_str() {
    quote! {
      #name = Some(Cow::Owned(r.read_text(#tag, &mut Vec::new())?));
    }
  } else {
    quote! {
      #name = Some(r.read_text(#tag, &mut Vec::new())?);
    }
  }
}

fn return_struct(s: &Struct) -> TokenStream {
  let struct_name = &s.name;

  macro_rules! return_flds {
    ($f:tt) => {
      s.$f
        .iter()
        .map(|f| {
          let name = &f.name;
          if f.ty.is_option().is_some() || f.ty.is_vec().is_some() || f.ty.is_bool() {
            quote! { #name, }
          } else {
            quote! { #name : #name.ok_or(Error::MissingField {
              name: String::from(stringify!(#struct_name)),
              field: String::from(stringify!(#name)),
            })?, }
          }
        }).collect::<Vec<_>>()
    };
  }

  let return_attr_flds = return_flds!(attr_flds);
  let return_child_flds = return_flds!(child_flds);
  let return_text_fld = return_flds!(text_fld);
  let return_flat_empty_flds = return_flds!(flat_empty_flds);
  let return_flat_empty_attr_flds = return_flds!(flat_empty_attr_flds);
  let return_flat_text_flds = return_flds!(flat_text_flds);

  quote! {
    #( #return_attr_flds )*
    #( #return_child_flds )*
    #( #return_text_fld )*
    #( #return_flat_empty_flds )*
    #( #return_flat_empty_attr_flds )*
    #( #return_flat_text_flds )*
  }
}

fn read_enum(e: &Enum) -> TokenStream {
  let enum_name = &e.name;

  let start_tags: &Vec<_> = &e
    .start_elem_vars
    .iter()
    .map(|v| {
      let tag = bytes_str!(v.tag);
      let name = &v.name;
      let ty = &v.ty.get_ident();
      quote!{ #tag => return #ty::read(r, Some(bs)).map(|p| #enum_name::#name(p)), }
    })
    .collect();

  let empty_tags: &Vec<_> = &e
    .empty_elem_vars
    .iter()
    .map(|v| {
      let tag = bytes_str!(v.tag);
      let name = &v.name;
      let ty = &v.ty.get_ident();
      quote!{ #tag => return #ty::read(r, Some(bs)).map(|p| #enum_name::#name(p)), }
    })
    .collect();

  quote! {
    if let Some(bs) = bs {
      match bs.name() {
        #( #start_tags )*
        #( #empty_tags )*
        _ => return Err(Error::UnexpectedTag {
          expected: String::from(stringify!( #( #empty_tags ),* #( #start_tags ),* )),
          found: String::from_utf8(bs.name().to_vec())?,
        }),
      }
    } else {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::Start(ref bs) => {
            match bs.name() {
              #( #start_tags )*
              _ => return Err(Error::UnexpectedTag {
                expected: String::from(stringify!( #( #start_tags ),* )),
                found: String::from_utf8(bs.name().to_vec())?,
              }),
            }
          },
          Event::Empty(ref bs) => {
            match bs.name() {
              #( #empty_tags )*
              _ => return Err(Error::UnexpectedTag {
                expected: String::from(stringify!( #( #empty_tags ),* )),
                found: String::from_utf8(bs.name().to_vec())?,
              }),
            }
          },
          Event::Eof => return Err(Error::UnexpectedEof),
          _ => (),
        }
        buf.clear();
      }
    }
  }
}

use proc_macro2::TokenStream;
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
    ($t:tt) => {{
      let names = s.$t.iter().map(|f| &f.name);
      let values = s.$t.iter().map(|f| f.ty.init_value());
      quote! { #(let mut #names = #values;)* }
    }};
  }

  let init_attr_flds = init_fld!(attr_flds);
  let init_child_flds = init_fld!(child_flds);
  let init_text_fld = init_fld!(text_fld);
  let init_flat_empty_flds = init_fld!(flat_empty_flds);
  let init_flat_empty_attr_flds = init_fld!(flat_empty_attr_flds);
  let init_flat_text_flds = init_fld!(flat_text_flds);

  quote! {
    #init_attr_flds
    #init_child_flds
    #init_text_fld
    #init_flat_empty_flds
    #init_flat_empty_attr_flds
    #init_flat_text_flds
  }
}

fn set_attrs(s: &Struct) -> Option<TokenStream> {
  if s.attr_flds.is_empty() {
    return None;
  }

  let tag = bytes_str!(s.tag);
  let name = &s.name;

  let field_names = s.attr_flds.iter().map(|f| &f.name);
  let field_tags = s.attr_flds.iter().map(|f| bytes_str!(f.attr));
  let field_values = s.attr_flds.iter().map(|f| f.ty.parse_attr_value());

  let loop_attrs = quote! {
    for attr in bs.attributes().filter_map(|a| a.ok()) {
      match attr.key {
        #(#field_tags => #field_names = Some(#field_values),)*
        k => info!(
          "Unhandled attribute {} when parsing {}.",
          String::from_utf8_lossy(k),
          stringify!(#name)
        ),
      }
    }
  };

  Some(quote! {
    if let Some(bs) = bs {
      #loop_attrs
    } else {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::Start(ref bs) | Event::Empty(ref bs) => {
            if bs.name() == #tag {
              #loop_attrs
              break;
            } else {
              return Err(Error::UnexpectedTag {
                expected: stringify!(#tag),
                found: String::from_utf8(bs.name().to_vec())?,
              });
            }
          },
          Event::Eof => return Err(Error::UnexpectedEof),
          _ => (),
        }
        buf.clear();
      }
    }
  })
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
    })
    .collect();

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

fn set_text(s: &Struct) -> Option<TokenStream> {
  match &s.text_fld {
    Some(field) => {
      let name = &field.name;
      let tag = bytes_str!(s.tag);

      Some(if field.ty.is_cow_str() {
        quote! {
          #name = Some(Cow::Owned(r.read_text(#tag, &mut Vec::new())?));
        }
      } else {
        quote! {
          #name = Some(r.read_text(#tag, &mut Vec::new())?);
        }
      })
    }
    None => None,
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
            quote! {
              #name: #name.ok_or(Error::MissingField {
                name: stringify!(#struct_name),
                field: stringify!(#name),
              })?,
            }
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
  use std::iter::repeat;

  let name = repeat(&e.name);
  let name1 = name.clone();

  let start_tags = e.start_elem_vars.iter().map(|v| bytes_str!(v.tag));
  let start_tags1 = start_tags.clone();
  let start_names = e.start_elem_vars.iter().map(|v| &v.name);
  let start_tys = e.start_elem_vars.iter().map(|v| v.ty.get_ident());

  let empty_tags = e.empty_elem_vars.iter().map(|v| bytes_str!(v.tag));
  let empty_tags1 = empty_tags.clone();
  let empty_names = e.empty_elem_vars.iter().map(|v| &v.name);
  let empty_tys = e.empty_elem_vars.iter().map(|v| v.ty.get_ident());

  let match_name = quote! {
    return match bs.name() {
      #(#start_tags => #start_tys::read(r, Some(bs)).map(#name::#start_names),)*
      #(#empty_tags => #empty_tys::read(r, Some(bs)).map(#name1::#empty_names),)*
      _ => Err(Error::UnexpectedTag {
        expected: stringify!(#(#start_tags1),* #(#empty_tags1),*),
        found: String::from_utf8(bs.name().to_vec())?,
      }),
    }
  };

  quote! {
    if let Some(bs) = bs {
      #match_name
    } else {
      let mut buf = Vec::new();
      loop {
        match r.read_event(&mut buf)? {
          Event::Start(ref bs) | Event::Empty(ref bs) => #match_name,
          Event::Eof => return Err(Error::UnexpectedEof),
          _ => (),
        }
        buf.clear();
      }
    }
  }
}

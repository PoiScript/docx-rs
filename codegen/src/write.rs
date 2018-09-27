use proc_macro2::TokenStream;
use types::{Enum, Event, Item, Struct, TypeExt};

macro_rules! bytes_str {
  ($t:expr) => {
    ::syn::LitByteStr::new($t.value().as_bytes(), ::proc_macro2::Span::call_site())
  };
}

pub(crate) fn impl_write(item: &Item) -> TokenStream {
  match item {
    Item::Enum(e) => write_enum(&e),
    Item::Struct(s) => write_struct(&s),
  }
}

fn write_struct(s: &Struct) -> TokenStream {
  match s.event {
    Event::Start => {
      let write_start_event = write_start_event(&s);
      let write_end_event = write_end_event(&s);
      let write_text_event = write_text_event(&s);
      let write_children = write_children(&s);
      let write_flatten_text = write_flatten_text(&s);
      let write_flatten_empty = wirte_flatten_empty(&s);
      let write_flatten_empty_attr = write_flatten_empty_attr(&s);

      quote! {
        #write_start_event

        #write_text_event

        #( #write_children )*

        #( #write_flatten_text )*

        #( #write_flatten_empty )*

        #( #write_flatten_empty_attr )*

        #write_end_event

        Ok(())
      }
    }
    Event::Empty => {
      let wirte_empty_event = write_empty_event(&s);

      quote! {
        #wirte_empty_event

        Ok(())
      }
    }
  }
}

fn write_enum(e: &Enum) -> TokenStream {
  let enum_name = &e.name;

  macro_rules! names {
    ($t:tt) => {
      &e.$t
        .iter()
        .map(|v| {
          let var_name = &v.name;
          quote!{ #enum_name::#var_name(s) => s.write(w), }
        }).collect::<Vec<_>>()
    };
  }

  let text_flat_vars = names!(text_flat_vars);
  let empty_flat_vars = names!(empty_flat_vars);
  let start_elem_vars = names!(start_elem_vars);
  let empty_elem_vars = names!(empty_elem_vars);

  quote!{
    match self {
      #( #text_flat_vars )*
      #( #empty_flat_vars )*
      #( #start_elem_vars )*
      #( #empty_elem_vars )*
    }
  }
}

fn write_start_event(s: &Struct) -> TokenStream {
  let tag = bytes_str!(s.tag);
  let write_attrs = write_attrs(&s);

  let extend_attrs = &s.extend_attrs;

  quote! {
    let mut start= BytesStart::borrowed(#tag, #tag.len());

    #( #write_attrs )*

    #extend_attrs(&self, &mut start);

    w.write_event(Event::Start(start))?;
  }
}

fn write_end_event(s: &Struct) -> TokenStream {
  let tag = bytes_str!(s.tag);

  quote! {
    w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
  }
}

fn write_empty_event(s: &Struct) -> TokenStream {
  let tag = bytes_str!(s.tag);
  let write_attrs = write_attrs(&s);

  quote! {
    let mut start= BytesStart::borrowed(#tag, #tag.len());

    #( #write_attrs )*

    w.write_event(Event::Empty(start))?;
  }
}

fn write_text_event(s: &Struct) -> TokenStream {
  if let Some(f) = &s.text_fld {
    let name = &f.name;
    quote! {
      w.write_event(Event::Text(BytesText::from_plain_str(self.#name.as_ref())))?;
    }
  } else {
    quote!()
  }
}

fn write_attrs(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.attr_flds {
    let name = &f.name;
    let tag = &f.attr;

    if let Some(ty) = f.ty.is_option() {
      if ty.is_bool() || ty.is_usize() {
        result.push(quote!{
          if let Some(ref #name) = self.#name {
            start.push_attribute((#tag, #name.to_string().as_str()));
          }
        });
      } else {
        result.push(quote!{
          if let Some(ref #name) = self.#name {
            start.push_attribute((#tag, #name.as_ref()));
          }
        });
      }
    } else {
      if f.ty.is_bool() || f.ty.is_usize() {
        result.push(quote!{ start.push_attribute((#tag, self.#name.to_string().as_str())); });
      } else {
        result.push(quote!{ start.push_attribute((#tag, self.#name.as_ref())); });
      }
    };
  }

  result
}

fn write_children(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.child_flds {
    let name = &f.name;

    if f.ty.is_option().is_some() {
      result.push(quote! {
        if let Some(ref #name) = self.#name {
          #name.write(w)?;
        }
      });
    } else if f.ty.is_vec().is_some() {
      result.push(quote! {
        for #name in &self.#name {
          #name.write(w)?;
        }
      });
    } else {
      result.push(quote! {
        self.#name.write(w)?;
      });
    }
  }

  result
}

fn write_flatten_text(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.flat_text_flds {
    let name = &f.name;
    let tag = bytes_str!(f.tag);

    if f.ty.is_option().is_some() {
      result.push(quote! {
        if let Some(ref #name) = self.#name {
          w.write_event(Event::Start(BytesStart::borrowed(#tag, #tag.len())))?;
          w.write_event(Event::Text(BytesText::from_plain_str(#name.as_ref())))?;
          w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
        }
      });
    } else {
      result.push(quote! {
        w.write_event(Event::Start(BytesStart::borrowed(#tag, #tag.len())))?;
        w.write_event(Event::Text(BytesText::from_plain_str(self.#name.as_ref())))?;
        w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
      });
    }
  }

  result
}

fn wirte_flatten_empty(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.flat_empty_flds {
    let name = &f.name;
    let tag = bytes_str!(f.tag);

    result.push(quote! {
      if self.#name {
        w.write_event(Event::Empty(BytesStart::borrowed(#tag, #tag.len())))?;
      }
    });
  }

  result
}

fn write_flatten_empty_attr(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in &s.flat_empty_attr_flds {
    let name = &f.name;
    let tag = bytes_str!(f.tag);
    let key = &f.attr;

    if let Some(ty) = f.ty.is_option() {
      let path = if ty.is_usize() || ty.is_bool() {
        quote!{ #name.to_string().as_str() }
      } else {
        quote!{ #name.as_ref() }
      };
      result.push(quote! {
        if let Some(ref #name) = self.#name {
          let mut start= BytesStart::borrowed(#tag, #tag.len());
          start.push_attribute((#key, #path));
          w.write_event(Event::Empty(start))?;
        }
      });
    } else {
      let path = if f.ty.is_usize() || f.ty.is_bool() {
        quote!{ self.#name.to_string().as_str() }
      } else {
        quote!{ self.#name.as_ref() }
      };
      result.push(quote! {
        let mut start= BytesStart::borrowed(#tag, #tag.len());
          start.push_attribute((#key, #path));
        w.write_event(Event::Empty(start))?;
      });
    }
  }

  result
}

use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use std::iter;
use types::{Enum, FieldType, Struct};

pub(crate) fn impl_write_struct(s: &Struct) -> TokenStream {
  let tag = &s.attrs.value;

  let attrs = write_attrs(&s);
  let start_event = write_start_event(&s);
  let chidren = write_children(&s);
  let end_event = write_end_event(&s, &tag);

  quote! {
    let mut start= BytesStart::borrowed(#tag.as_bytes(), #tag.len());

    #( #attrs )*

    #start_event

    #( #chidren )*

    #end_event

    Ok(())
  }
}

fn write_attrs(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in s.filter_field("attr") {
    let tag = &f.attrs.value;
    let name = Ident::new(&f.name, Span::call_site());

    let ident = match f.get_ty() {
      FieldType::String => if f.is_option {
        quote!{ #name as &str }
      } else {
        quote!{ self.#name.as_ref() }
      },
      FieldType::Slices | FieldType::Cow => quote!{ self.#name },
      FieldType::Others(_) => quote!{ self.#name.as_str() },
    };

    result.push(if f.is_option {
      quote!{
        if let Some(ref #name) = self.#name {
          start.push_attribute((#tag, #ident));
        }
      }
    } else {
      quote!{  start.push_attribute((#tag, #ident)); }
    });
  }

  result
}

fn write_start_event(s: &Struct) -> TokenStream {
  if s.attrs.key == "parent" || s.attrs.key == "text" {
    quote!{
      w.write_event(Event::Start(start))?;
    }
  } else {
    quote!{
      w.write_event(Event::Empty(start))?;
    }
  }
}

fn write_children(s: &Struct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  if s.attrs.key == "parent" {
    for f in s.filter_field("child") {
      let name = Ident::new(&f.name, Span::call_site());

      if f.is_option {
        result.push(quote! {
          if let Some(ref #name) = self.#name {
            #name.write(w)?;
          }
        });
      } else if f.is_vec {
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
  } else if s.attrs.key == "text" {
    let text = Ident::new(&s.find_field("text").name, Span::call_site());

    result.push(quote! {
      w.write_event(Event::Text(BytesText::from_plain_str(self.#text.as_ref())))?;
    });
  }

  result
}

fn write_end_event(s: &Struct, tag: &String) -> TokenStream {
  if s.attrs.key == "parent" || s.attrs.key == "text" {
    quote!{
      w.write_event(Event::End(BytesEnd::borrowed(#tag.as_bytes())))?;
    }
  } else {
    quote!()
  }
}

pub(crate) fn impl_write_enum(e: &Enum) -> TokenStream {
  let fields: &Vec<_> = &e
    .fields
    .iter()
    .map(|f| Ident::new(&f.name, Span::call_site()))
    .collect();

  let names: Vec<_> = iter::repeat(Ident::new(&e.name, Span::call_site()))
    .take(fields.len())
    .collect();

  quote!{
    match self {
      #( #names::#fields(s) => s.write(w), )*
    }
  }
}

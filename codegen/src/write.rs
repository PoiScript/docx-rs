use proc_macro2::TokenStream;
use types::Event;
use types::{Item, ItemEnum, ItemStruct};

pub(crate) fn impl_write(item: &Item) -> TokenStream {
  match item {
    Item::Enum(e) => write_enum(&e),
    Item::Struct(s) => write_struct(&s),
  }
}

fn write_struct(s: &ItemStruct) -> TokenStream {
  match s.config.event {
    Event::Start => {
      let write_start_event = write_start_event(&s);
      let write_end_event = write_end_event(&s);
      let write_text_event = write_text_event(&s);
      let write_children = write_children(&s);
      let write_flattern_text = write_flattern_text(&s);

      quote! {
        #write_start_event

        #write_text_event

        #( #write_children )*

        #( #write_flattern_text )*

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

fn write_enum(e: &ItemEnum) -> TokenStream {
  use std::iter::repeat;

  let variant_names: &Vec<_> = &e.variants.iter().map(|f| &f.name).collect();

  let enum_names = repeat(&e.name);

  quote!{
    match self {
      #( #enum_names::#variant_names(s) => s.write(w), )*
    }
  }
}

fn write_start_event(s: &ItemStruct) -> TokenStream {
  let tag = &s.config.tag;
  let write_attrs = write_attrs(&s);

  let extend_attrs = &s.config.extend_attrs;

  quote! {
    let mut start= BytesStart::borrowed(#tag, #tag.len());

    #( #write_attrs )*

    #extend_attrs(&self, &mut start);

    w.write_event(Event::Start(start))?;
  }
}

fn write_end_event(s: &ItemStruct) -> TokenStream {
  let tag = &s.config.tag;

  quote! {
    w.write_event(Event::End(BytesEnd::borrowed(#tag)))?;
  }
}

fn write_empty_event(s: &ItemStruct) -> TokenStream {
  let tag = &s.config.tag;
  let write_attrs = write_attrs(&s);

  quote! {
    let mut start= BytesStart::borrowed(#tag, #tag.len());

    #( #write_attrs )*

    w.write_event(Event::Empty(start))?;
  }
}

fn write_text_event(s: &ItemStruct) -> TokenStream {
  if let Some(f) = s.fields.iter().find(|f| f.config.is_text) {
    let name = &f.name;
    quote! {
      w.write_event(Event::Text(BytesText::from_plain_str(self.#name.as_ref())))?;
    }
  } else {
    quote!()
  }
}

fn write_attrs(s: &ItemStruct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  for f in s.fields.iter().filter(|f| f.config.is_attr) {
    let name = &f.name.clone().unwrap();
    let tag = &f.config.attr.clone().unwrap();

    if f.is_option().is_some() {
      result.push(quote!{
        if let Some(ref #name) = self.#name {
          start.push_attribute((#tag, #name as &str));
        }
      });
    } else {
      result.push(quote!{ start.push_attribute((#tag, self.#name.as_ref() as &str)); });
    };
  }

  result
}

fn write_children(s: &ItemStruct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  let child_fields = s.fields.iter().filter(|f| f.config.is_child);

  for f in child_fields {
    let name = &f.name;

    if f.is_option().is_some() {
      result.push(quote! {
        if let Some(ref #name) = self.#name {
          #name.write(w)?;
        }
      });
    } else if f.is_vec().is_some() {
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

fn write_flattern_text(s: &ItemStruct) -> Vec<TokenStream> {
  let mut result = Vec::new();

  let flattern_text_fields = s.fields.iter().filter(|f| f.config.is_flattern_text);

  for f in flattern_text_fields {
    let name = &f.name.clone().unwrap();
    let tag = &f
      .config
      .tag
      .clone()
      .expect(&format!("Tag attribute not found in field {}", name));

    if f.is_option().is_some() {
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

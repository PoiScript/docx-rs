use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn;
use syn::Lit::*;
use syn::Meta::*;
use syn::NestedMeta::Meta;

#[cfg_attr(feature = "cargo-clippy", allow(large_enum_variant))]
pub(crate) enum Item {
  Enum(Enum),
  Struct(Struct),
}

pub(crate) enum Event {
  Start,
  Empty,
}

pub(crate) struct Struct {
  pub name: syn::Ident,
  pub generics: syn::Generics,
  pub event: Event,
  pub tag: syn::LitStr,
  pub extend_attrs: Option<syn::Ident>,
  pub attr_flds: Vec<AttrField>,
  pub child_flds: Vec<ChildField>,
  pub text_fld: Option<TextField>,
  pub flat_empty_flds: Vec<EmptyFlatField>,
  pub flat_empty_attr_flds: Vec<EmptyFlatAttrField>,
  pub flat_text_flds: Vec<TextFlatField>,
}

pub(crate) struct AttrField {
  pub attr: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct ChildField {
  pub tags: Vec<syn::LitStr>,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct TextField {
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct TextFlatField {
  pub tag: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct EmptyFlatField {
  pub tag: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct EmptyFlatAttrField {
  pub attr: syn::LitStr,
  pub tag: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

impl Struct {
  pub fn parse(
    data: &syn::DataStruct,
    attrs: &[syn::Attribute],
    ident: &syn::Ident,
    generics: &syn::Generics,
  ) -> Struct {
    let mut event = None;
    let mut tag = None;
    let mut extend_attrs = None;

    for meta_items in attrs.iter().filter_map(get_xml_meta_items) {
      for meta_item in meta_items {
        match meta_item {
          Meta(NameValue(ref m)) if m.ident == "event" => {
            if let Str(ref lit) = m.lit {
              event = Some(match lit.value().as_ref() {
                "Start" => Event::Start,
                "Empty" => Event::Empty,
                _ => panic!("Unknown event '{}'", lit.value()),
              });
            }
          }
          Meta(NameValue(ref m)) if m.ident == "tag" => {
            if let Str(ref lit) = m.lit {
              tag = Some(lit.clone());
            }
          }
          Meta(NameValue(ref m)) if m.ident == "extend_attrs" => {
            if let Str(ref lit) = m.lit {
              extend_attrs = Some(syn::Ident::new(&lit.value(), Span::call_site()));
            }
          }
          _ => (),
        }
      }
    }

    let mut attr_flds = Vec::new();
    let mut child_flds = Vec::new();
    let mut text_fld = None;
    let mut flat_empty_flds = Vec::new();
    let mut flat_text_flds = Vec::new();
    let mut flat_empty_attr_flds = Vec::new();

    for field in data.fields.iter() {
      let name = field.ident.clone().unwrap();
      let ty = field.ty.clone();
      let mut attr = None;
      let mut child = false;
      let mut flat_empty = false;
      let mut flat_text = false;
      let mut tags = Vec::new();
      let mut text = false;

      for meta_items in field.attrs.iter().filter_map(get_xml_meta_items) {
        for meta_item in meta_items {
          match meta_item {
            Meta(NameValue(ref m)) if m.ident == "attr" => {
              if let Str(ref lit) = m.lit {
                attr = Some(lit.clone());
              }
            }
            Meta(NameValue(ref m)) if m.ident == "tag" => {
              if let Str(ref lit) = m.lit {
                tags.push(lit.clone());
              }
            }
            Meta(Word(ref w)) if w == "flatten_text" => {
              flat_text = true;
            }
            Meta(Word(ref w)) if w == "flatten_empty" => {
              flat_empty = true;
            }
            Meta(Word(ref w)) if w == "text" => {
              text = true;
            }
            Meta(Word(ref w)) if w == "child" => {
              child = true;
            }
            _ => panic!(
              "Unkown attribute when parsing field {}:\n{}.",
              name,
              field.into_token_stream()
            ),
          }
        }
      }

      match (attr, child, flat_text, flat_empty, tags.len(), text) {
        (Some(attr), false, false, false, 0, false) => attr_flds.push(AttrField { attr, name, ty }),
        (None, true, false, false, 1...10, false) => child_flds.push(ChildField { tags, name, ty }),
        (None, false, false, false, 0, true) => text_fld = Some(TextField { name, ty }),
        (None, false, true, false, 1, false) => {
          let tag = tags.pop().unwrap();
          flat_text_flds.push(TextFlatField { tag, name, ty });
        }
        (None, false, false, true, 1, false) => {
          let tag = tags.pop().unwrap();
          flat_empty_flds.push(EmptyFlatField { tag, name, ty });
        }
        (Some(attr), false, false, true, 1, false) => {
          let tag = tags.pop().unwrap();
          flat_empty_attr_flds.push(EmptyFlatAttrField {
            tag,
            attr,
            name,
            ty,
          });
        }
        _ => panic!(
          "Unkown attribute when parsing field {}:\n{}.",
          name,
          field.into_token_stream()
        ),
      }
    }

    Struct {
      event: event.unwrap_or_else(|| panic!("No event attribute found for {}", ident)),
      tag: tag.unwrap_or_else(|| panic!("No tag attribute found for {}", ident)),
      extend_attrs,
      name: ident.clone(),
      generics: generics.clone(),
      attr_flds,
      child_flds,
      text_fld,
      flat_empty_flds,
      flat_empty_attr_flds,
      flat_text_flds,
    }
  }
}

pub(crate) struct Enum {
  pub name: syn::Ident,
  pub generics: syn::Generics,
  pub text_flat_vars: Vec<Variant>,
  pub empty_flat_vars: Vec<Variant>,
  pub start_elem_vars: Vec<Variant>,
  pub empty_elem_vars: Vec<Variant>,
}

impl Enum {
  pub fn parse(data: &syn::DataEnum, ident: &syn::Ident, generics: &syn::Generics) -> Enum {
    let mut text_flat_vars = Vec::new();
    let mut empty_flat_vars = Vec::new();
    let mut start_elem_vars = Vec::new();
    let mut empty_elem_vars = Vec::new();

    for variant in &data.variants {
      let name = variant.ident.clone();
      let ty = variant.fields.iter().nth(0).unwrap().ty.clone();
      let mut event = None;
      let mut flat_empty = false;
      let mut flat_text = false;
      let mut tags = Vec::new();
      for meta_items in variant.attrs.iter().filter_map(get_xml_meta_items) {
        for meta_item in meta_items {
          match meta_item {
            Meta(NameValue(ref m)) if m.ident == "tag" => {
              if let Str(ref lit) = m.lit {
                tags.push(lit.clone());
              }
            }
            Meta(Word(ref w)) if w == "flatten_text" => {
              flat_text = true;
            }
            Meta(Word(ref w)) if w == "flatten_empty" => {
              flat_empty = true;
            }
            Meta(NameValue(ref m)) if m.ident == "event" => {
              if let Str(ref lit) = m.lit {
                event = Some(lit.value());
              }
            }
            _ => panic!("Unkown attribute when parsing variant {}.", name),
          }
        }
      }

      let tag = tags.pop().unwrap();
      match (event, flat_text, flat_empty) {
        (Some(e), false, false) => {
          if e == "Start" {
            start_elem_vars.push(Variant { tag, name, ty });
          } else if e == "Empty" {
            empty_elem_vars.push(Variant { tag, name, ty });
          }
        }
        (None, true, false) => text_flat_vars.push(Variant { tag, name, ty }),
        (None, false, true) => empty_flat_vars.push(Variant { tag, name, ty }),
        _ => panic!("Unkown field type when parsing field {}.", name),
      }
    }

    Enum {
      name: ident.clone(),
      generics: generics.clone(),
      text_flat_vars,
      empty_flat_vars,
      start_elem_vars,
      empty_elem_vars,
    }
  }
}

fn get_xml_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
  if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "xml" {
    match attr.interpret_meta() {
      Some(syn::Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
      _ => None,
    }
  } else {
    None
  }
}

pub(crate) struct Variant {
  pub tag: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub trait TypeExt {
  fn is_option(&self) -> Option<&Self>;
  fn is_cow_str(&self) -> bool;
  fn is_vec(&self) -> Option<&Self>;
  fn is_bool(&self) -> bool;
  fn is_string(&self) -> bool;
  fn is_usize(&self) -> bool;
  fn init_value(&self) -> TokenStream;
  fn parse_attr_value(&self) -> TokenStream;
  fn get_ident(&self) -> Option<syn::Ident>;
}

impl TypeExt for syn::Type {
  fn is_option(&self) -> Option<&Self> {
    let path = match self {
      syn::Type::Path(ref ty) => &ty.path,
      _ => {
        return None;
      }
    };
    let seg = match path.segments.last() {
      Some(seg) => seg.into_value(),
      None => {
        return None;
      }
    };
    let args = match seg.arguments {
      syn::PathArguments::AngleBracketed(ref bracketed) => &bracketed.args,
      _ => {
        return None;
      }
    };
    if seg.ident == "Option" && args.len() == 1 {
      match args[0] {
        syn::GenericArgument::Type(ref arg) => Some(arg),
        _ => None,
      }
    } else {
      None
    }
  }

  fn is_cow_str(&self) -> bool {
    let path = match self {
      syn::Type::Path(ref ty) => &ty.path,
      _ => {
        return false;
      }
    };
    let seg = match path.segments.last() {
      Some(seg) => seg.into_value(),
      None => {
        return false;
      }
    };
    let args = match seg.arguments {
      syn::PathArguments::AngleBracketed(ref bracketed) => &bracketed.args,
      _ => {
        return false;
      }
    };
    if args.len() != 2 {
      return false;
    }
    let ty = match (&args[0], &args[1]) {
      (&syn::GenericArgument::Lifetime(_), &syn::GenericArgument::Type(ref arg)) => arg,
      _ => return false,
    };
    seg.ident == "Cow" && match *ty {
      syn::Type::Path(ref ty) => {
        ty.qself.is_none() && ty.path.segments.len() == 1 && ty.path.segments[0].ident == "str"
      }
      _ => false,
    }
  }

  fn is_vec(&self) -> Option<&Self> {
    let path = match self {
      syn::Type::Path(ref ty) => &ty.path,
      _ => {
        return None;
      }
    };
    let seg = match path.segments.last() {
      Some(seg) => seg.into_value(),
      None => {
        return None;
      }
    };
    let args = match seg.arguments {
      syn::PathArguments::AngleBracketed(ref bracketed) => &bracketed.args,
      _ => {
        return None;
      }
    };
    if seg.ident == "Vec" && args.len() == 1 {
      match args[0] {
        syn::GenericArgument::Type(ref arg) => Some(arg),
        _ => None,
      }
    } else {
      None
    }
  }

  // Specifies how to initialize this field by its type
  fn init_value(&self) -> TokenStream {
    if self.is_vec().is_some() {
      quote!(Vec::new())
    } else if self.is_bool() {
      quote!(false)
    } else {
      quote!(None)
    }
  }

  // Specifies how to parse attributes value (attr.value) by its type
  fn parse_attr_value(&self) -> TokenStream {
    let ty = self.is_option().unwrap_or(&self);

    if ty.is_string() {
      quote!(String::from_utf8(attr.value.into_owned().to_vec())?)
    } else if ty.is_cow_str() {
      quote!(Cow::Owned(String::from_utf8(
        attr.value.into_owned().to_vec()
      )?))
    } else if ty.is_bool() {
      quote! {{
        let value = ::std::str::from_utf8(attr.value.borrow())?;
        bool::from_str(value).or(usize::from_str(value).map(|v| v != 0))?
      }}
    } else {
      let ty = ty.get_ident();
      quote!(#ty::from_str(::std::str::from_utf8(attr.value.borrow())?)?)
    }
  }

  fn is_string(&self) -> bool {
    self.get_ident().map_or(false, |ty| ty == "String")
  }

  fn is_bool(&self) -> bool {
    self.get_ident().map_or(false, |ty| ty == "bool")
  }

  fn is_usize(&self) -> bool {
    self.get_ident().map_or(false, |ty| ty == "usize")
  }

  fn get_ident(&self) -> Option<syn::Ident> {
    match self {
      syn::Type::Path(ref ty) => ty
        .path
        .segments
        .last()
        .map(|seg| seg.into_value().ident.clone()),
      syn::Type::Reference(ref ty) => ty.elem.get_ident(),
      _ => None,
    }
  }
}

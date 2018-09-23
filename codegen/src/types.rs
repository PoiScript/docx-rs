use proc_macro2::Span;
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
  pub flat_empty_flds: Vec<EmptyFlatternField>,
  pub flat_text_flds: Vec<TextFlatternField>,
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

pub(crate) struct TextFlatternField {
  pub tag: syn::LitStr,
  pub name: syn::Ident,
  pub ty: syn::Type,
}

pub(crate) struct EmptyFlatternField {
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
            Meta(Word(ref w)) if w == "flattern_text" => {
              flat_text = true;
            }
            Meta(Word(ref w)) if w == "flattern_empty" => {
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
        (None, false, true, false, 1, false) => flat_text_flds.push(TextFlatternField {
          tag: tags.pop().unwrap(),
          name,
          ty,
        }),
        (Some(attr), false, false, true, 1, false) => flat_empty_flds.push(EmptyFlatternField {
          tag: tags.pop().unwrap(),
          attr,
          name,
          ty,
        }),
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
            Meta(Word(ref w)) if w == "flattern_text" => {
              flat_text = true;
            }
            Meta(Word(ref w)) if w == "flattern_empty" => {
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
    if seg.ident == "Cow" {
      match *ty {
        syn::Type::Path(ref ty) => {
          ty.qself.is_none() && ty.path.segments.len() == 1 && ty.path.segments[0].ident == "str"
        }
        _ => false,
      }
    } else {
      false
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

  fn get_ident(&self) -> Option<syn::Ident> {
    let path = match self {
      syn::Type::Path(ref ty) => &ty.path,
      _ => {
        return None;
      }
    };
    path
      .segments
      .last()
      .map(|seg| seg.into_value().ident.clone())
  }
}

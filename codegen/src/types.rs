use proc_macro2::Span;
use syn;
use syn::Lit::*;
use syn::Meta::*;
use syn::NestedMeta::Meta;
use syn::Type;
use syn::*;

#[derive(Debug)]
pub(crate) enum Item {
  Enum(ItemEnum),
  Struct(ItemStruct),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Event {
  Start,
  Empty,
}

#[derive(Debug)]
pub(crate) struct StructConfig {
  pub event: Event,
  pub tag: LitByteStr,
  pub extend_attrs: Option<Ident>,
}

#[derive(Debug)]
pub(crate) struct ItemStruct {
  pub config: StructConfig,
  pub name: Ident,
  pub generics: Generics,
  pub fields: Vec<Field>,
}

impl ItemStruct {
  pub fn parse(
    data: &DataStruct,
    attrs: &Vec<Attribute>,
    ident: &Ident,
    generics: &Generics,
  ) -> ItemStruct {
    ItemStruct {
      config: ItemStruct::parse_attrs(ident, attrs),
      name: ident.clone(),
      generics: generics.clone(),
      fields: data.fields.iter().map(|f| Field::parse(f)).collect(),
    }
  }

  fn parse_attrs(name: &Ident, attrs: &Vec<syn::Attribute>) -> StructConfig {
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
                _ => unreachable!(),
              });
            }
          }
          Meta(NameValue(ref m)) if m.ident == "tag" => {
            if let ByteStr(ref lit) = m.lit {
              tag = Some(lit.clone());
            } else if let Str(ref lit) = m.lit {
              // convert str to byte str here,
              // 'cause rust stable doesn't support byte str in attribute now
              tag = Some(LitByteStr::new(
                &lit.value().as_bytes().to_vec(),
                Span::call_site(),
              ));
            }
          }
          Meta(NameValue(ref m)) if m.ident == "extend_attrs" => {
            if let Str(ref lit) = m.lit {
              extend_attrs = Some(Ident::new(&lit.value(), Span::call_site()));
            }
          }
          _ => (),
        }
      }
    }

    StructConfig {
      event: event.expect(&format!("No event attribute found for {}", name)),
      tag: tag.expect(&format!("No tag attribute found for {}", name)),
      extend_attrs,
    }
  }
}

#[derive(Debug)]
pub(crate) struct ItemEnum {
  pub name: Ident,
  pub variants: Vec<Variant>,
  pub generics: Generics,
}

impl ItemEnum {
  pub fn parse(
    data: &DataEnum,
    _: &Vec<Attribute>,
    ident: &Ident,
    generics: &Generics,
  ) -> ItemEnum {
    ItemEnum {
      name: ident.clone(),
      generics: generics.clone(),
      variants: data.variants.iter().map(|v| Variant::parse(v)).collect(),
    }
  }
}

#[derive(Debug, Default)]
pub(crate) struct FieldConfig {
  pub attr: Option<LitStr>,
  pub is_attr: bool,
  pub is_child: bool,
  pub is_flattern_text: bool,
  pub is_text: bool,
  pub tag: Option<LitByteStr>,
}

#[derive(Debug)]
pub(crate) struct Field {
  pub config: FieldConfig,
  pub name: Option<Ident>,
  pub ty: Type,
}

impl Field {
  fn parse(field: &syn::Field) -> Field {
    Field {
      config: Field::parse_attrs(&field.attrs),
      name: field.ident.clone(),
      ty: field.ty.clone(),
    }
  }

  fn parse_attrs(attrs: &Vec<syn::Attribute>) -> FieldConfig {
    let mut config = FieldConfig::default();

    for meta_items in attrs.iter().filter_map(get_xml_meta_items) {
      for meta_item in meta_items {
        match meta_item {
          Meta(NameValue(ref m)) if m.ident == "attr" => {
            if let Str(ref lit) = m.lit {
              config.attr = Some(lit.clone());
              config.is_attr = true;
            }
          }
          Meta(NameValue(ref m)) if m.ident == "tag" => {
            if let ByteStr(ref lit) = m.lit {
              config.tag = Some(lit.clone());
            } else if let Str(ref lit) = m.lit {
              // convert str to byte str here,
              // 'cause rust stable doesn't support byte str in attribute now
              config.tag = Some(LitByteStr::new(
                &lit.value().as_bytes().to_vec(),
                Span::call_site(),
              ));
            }
          }
          Meta(Word(ref w)) if w == "flattern_text" => {
            config.is_flattern_text = true;
          }
          Meta(Word(ref w)) if w == "text" => {
            config.is_text = true;
          }
          Meta(Word(ref w)) if w == "child" => {
            config.is_child = true;
          }
          _ => (),
        }
      }
    }

    config
  }

  pub fn is_option(&self) -> Option<&Type> {
    let path = match self.ty {
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

  pub fn is_cow_str(&self) -> bool {
    let path = match self.ty {
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
    let ty = match (&args[0], &args[1]) {
      (&syn::GenericArgument::Lifetime(_), &syn::GenericArgument::Type(ref arg)) => arg,
      _ => return false,
    };
    seg.ident == "Cow" && args.len() == 2 && match *ty {
      syn::Type::Path(ref ty) => {
        ty.qself.is_none() && ty.path.segments.len() == 1 && ty.path.segments[0].ident == "str"
      }
      _ => false,
    }
  }

  pub fn is_vec(&self) -> Option<&Type> {
    let path = match self.ty {
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
}

#[derive(Debug)]
pub(crate) struct VariantConfig {
  pub event: Event,
  pub tag: LitByteStr,
}

#[derive(Debug)]
pub(crate) struct Variant {
  pub config: VariantConfig,
  pub name: Ident,
  pub field: Field,
}

impl Variant {
  pub fn parse(variant: &syn::Variant) -> Variant {
    let field = variant.fields.iter().nth(0).unwrap();
    Variant {
      config: Variant::parse_variant_attrs(&variant),
      name: variant.ident.clone(),
      field: Field::parse(field),
    }
  }

  fn parse_variant_attrs(variant: &syn::Variant) -> VariantConfig {
    let mut event = None;
    let mut tag = None;

    for meta_items in variant.attrs.iter().filter_map(get_xml_meta_items) {
      for meta_item in meta_items {
        match meta_item {
          Meta(NameValue(ref m)) if m.ident == "event" => {
            if let Str(ref lit) = m.lit {
              event = Some(match lit.value().as_ref() {
                "Start" => Event::Start,
                "Empty" => Event::Empty,
                _ => unreachable!(),
              });
            }
          }
          Meta(NameValue(ref m)) if m.ident == "tag" => {
            if let ByteStr(ref lit) = m.lit {
              tag = Some(lit.clone());
            } else if let Str(ref lit) = m.lit {
              // convert str to byte str here,
              // 'cause rust stable doesn't support byte str in attribute now
              tag = Some(LitByteStr::new(
                &lit.value().as_bytes().to_vec(),
                Span::call_site(),
              ));
            }
          }
          _ => (),
        }
      }
    }

    VariantConfig {
      event: event.expect(&format!("No event attribute found for {}", variant.ident)),
      tag: tag.expect(&format!("No tag attribute found for {}", variant.ident)),
    }
  }
}

fn get_xml_meta_items(attr: &Attribute) -> Option<Vec<NestedMeta>> {
  use syn::Meta::List;

  if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "xml" {
    match attr.interpret_meta() {
      Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
      _ => None,
    }
  } else {
    None
  }
}

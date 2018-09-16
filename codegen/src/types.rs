use proc_macro2::{Ident, Span};
use regex::Regex;

#[derive(Debug)]
pub(crate) struct Attribute {
  pub key: String,
  pub value: String,
}

#[derive(Debug)]
pub(crate) struct Field {
  pub name: Ident,
  pub ty: Ident,
  pub attrs: Attribute,
  pub is_vec: bool,
  pub is_option: bool,
}

pub(crate) enum FieldType {
  String,
  Cow,
  Slices,
  Others(Ident),
}

impl Field {
  pub fn get_ty(&self) -> FieldType {
    let cow_re = Regex::new(r"Cow<'(\w+), str>").unwrap();
    let str_re = Regex::new(r"&'(\w) str").unwrap();

    if self.ty == "String" {
      FieldType::String
    } else if cow_re.is_match(&format!("{}", self.ty)) {
      FieldType::Cow
    } else if str_re.is_match(&format!("{}", self.ty)) {
      FieldType::Slices
    } else {
      FieldType::Others(self.ty.clone())
    }
  }
}

#[derive(Debug)]
pub(crate) struct Struct {
  pub name: Ident,
  pub fields: Vec<Field>,
  pub attrs: Attribute,
}

#[derive(Debug)]
pub(crate) struct Enum {
  pub name: Ident,
  pub fields: Vec<Field>,
}

macro_rules! filter_fields {
  ( $t:tt, $( $x:expr ),* ) => {
    $t
      .fields
      .iter()
      .filter(|f| $( f.attrs.key == $x ) || * )
      .collect()
  };
}

impl Struct {
  pub fn children(&self) -> Vec<&Field> {
    filter_fields!(self, "child")
  }

  pub fn texts(&self) -> Vec<&Field> {
    filter_fields!(self, "text")
  }

  pub fn texts_and_children(&self) -> Vec<&Field> {
    filter_fields!(self, "text", "child")
  }

  pub fn attrs(&self) -> Vec<&Field> {
    filter_fields!(self, "attr")
  }
}

impl Enum {
  pub fn texts_and_children(&self) -> Vec<&Field> {
    filter_fields!(self, "text", "child")
  }

  pub fn empties(&self) -> Vec<&Field> {
    filter_fields!(self, "empty")
  }
}

pub(crate) fn parse_enum(enum_str: String) -> Enum {
  let enum_re = Regex::new(r#"(:?pub )?enum (?P<name>.+) \{"#).unwrap();

  let filed_re = Regex::new(
    r#"#\[xml\((?P<key>[:\w]+) = "(?P<value>.+)"\)\]\n\s+(?P<name>.+)\s?\((?P<ty>.+)\),"#,
  ).unwrap();

  let fields: Vec<_> = filed_re
    .captures_iter(&enum_str)
    .map(|caps| Field {
      is_vec: false,
      is_option: false,
      ty: Ident::new(&caps["ty"], Span::call_site()),
      name: Ident::new(&caps["name"], Span::call_site()),
      attrs: Attribute {
        key: caps["key"].to_string(),
        value: caps["value"].to_string(),
      },
    }).collect();

  let caps = enum_re.captures(&enum_str).unwrap();

  Enum {
    name: Ident::new(&caps["name"], Span::call_site()),
    fields,
  }
}

pub(crate) fn parse_struct(struct_str: String) -> Struct {
  let struct_re =
    Regex::new(r#"#\[xml\((?P<key>.+) = "(?P<value>.+)"\)\]\n(:?pub )?struct (?P<name>.+) \{"#)
      .unwrap();

  let filed_re =
    Regex::new(r#"#\[xml\((?P<key>[:\w]+)(:? = )?(:?"(?P<value>.+)")?\)\]\n\s+(:?pub )(?P<name>.+): (?P<ty>.+),"#)
      .unwrap();

  let option_re = Regex::new(r#"Option<(?P<ty>.+)>"#).unwrap();

  let vec_re = Regex::new(r#"Vec<(?P<ty>.+)>"#).unwrap();

  let fields: Vec<_> = filed_re
    .captures_iter(&struct_str)
    .map(|caps| {
      let mut is_vec = false;
      let mut is_option = false;
      let mut ty = caps["ty"].to_string();

      if let Some(caps) = option_re.captures(&ty.clone()) {
        is_option = true;
        ty = caps["ty"].to_string();
      }

      if let Some(caps) = vec_re.captures(&ty.clone()) {
        is_vec = true;
        ty = caps["ty"].to_string();
      }

      Field {
        is_vec,
        is_option,
        ty: Ident::new(&ty, Span::call_site()),
        name: Ident::new(&caps["name"], Span::call_site()),
        attrs: Attribute {
          key: caps["key"].to_string(),
          value: caps
            .name("value")
            .map(|m| m.as_str().to_string())
            .unwrap_or("".to_string()),
        },
      }
    }).collect();

  let caps = struct_re.captures(&struct_str).unwrap();

  Struct {
    name: Ident::new(&caps["name"], Span::call_site()),
    fields,
    attrs: Attribute {
      key: caps["key"].to_string(),
      value: caps
        .name("value")
        .map(|m| m.as_str().to_string())
        .unwrap_or("".to_string()),
    },
  }
}

use regex::Regex;

#[derive(Debug)]
pub(crate) struct Attribute {
  pub key: String,
  pub value: String,
}

#[derive(Debug)]
pub(crate) struct Field {
  pub name: String,
  pub ty: String,
  pub attrs: Attribute,
}

pub(crate) enum FieldType {
  String,
  Cow,
  Str,
  Option { ty: String },
  Others { ty: String },
}

impl Field {
  pub fn get_ty(&self) -> FieldType {
    let cow_re = Regex::new(r"Cow<'(\w+), str>").unwrap();
    let option_re = Regex::new(r"Option<(.+)>").unwrap();
    let str_re = Regex::new(r"&'(\w) str").unwrap();

    if self.ty == "String" {
      FieldType::String
    } else if let Some(caps) = cow_re.captures(&self.ty) {
      FieldType::Cow
    } else if let Some(caps) = option_re.captures(&self.ty) {
      FieldType::Option {
        ty: caps[1].to_string(),
      }
    } else if let Some(caps) = str_re.captures(&self.ty) {
      FieldType::Str
    } else {
      FieldType::Others {
        ty: self.ty.clone(),
      }
    }
  }
}

#[derive(Debug)]
pub(crate) struct Structure {
  pub name: String,
  pub fields: Vec<Field>,
  pub attrs: Attribute,
}

impl Structure {
  pub fn find_field(&self, key: &'static str) -> &Field {
    self.fields.iter().find(|f| f.attrs.key == key).unwrap()
  }

  pub fn filter_field(&self, key: &'static str) -> Vec<&Field> {
    self
      .fields
      .iter()
      .filter(|f| f.attrs.key == key)
      .collect::<Vec<_>>()
  }
}

pub(crate) fn parse_struct(struct_str: String) -> Structure {
  let struct_re =
    Regex::new(r#"#\[xml\((?P<key>.+) = "(?P<value>.+)"\)\]\n(:?pub )?struct (?P<name>.+) \{"#)
      .unwrap();

  let filed_re =
    Regex::new(r#"#\[xml\((?P<key>[:\w]+)(:? = )?(:?"(?P<value>.+)")?\)\]\n\s+(:?pub )(?P<name>.+): (?P<ty>.+),"#)
      .unwrap();

  let fields: Vec<_> = filed_re
    .captures_iter(&struct_str)
    .map(|caps| Field {
      name: caps["name"].to_string(),
      ty: caps["ty"].to_string(),
      attrs: Attribute {
        key: caps["key"].to_string(),
        value: caps
          .name("value")
          .map(|m| m.as_str().to_string())
          .unwrap_or("".to_string()),
      },
    }).collect();

  let caps = struct_re.captures(&struct_str).unwrap();

  Structure {
    name: caps["name"].to_string(),
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

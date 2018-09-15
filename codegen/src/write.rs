use types::{Enum, FieldType, Struct};

pub(crate) fn impl_write_struct(s: &Struct) -> String {
  let mut result = String::with_capacity(1000);

  let event = match s.attrs.key.as_ref() {
    "parent" | "text" => "Start",
    "empty" => "Empty",
    _ => unreachable!("element should be one of the following types: parent, text or empty."),
  };

  result.push_str(&format!(
    r#"let mut start= BytesStart::borrowed(b"{}", {});
"#,
    s.attrs.value,
    s.attrs.value.len()
  ));

  for f in s.filter_field("attr") {
    if f.is_option {
      result.push_str(&format!("if let Some(ref {0}) = self.{0} {{\n", f.name));
    }
    match f.get_ty() {
      FieldType::String => result.push_str(&format!(
        "start.push_attribute((\"{}\", {}));\n",
        f.attrs.value,
        if f.is_option {
          format!("{} as &str", f.name)
        } else {
          format!("self.{}.as_ref()", f.name)
        }
      )),
      FieldType::Slices | FieldType::Cow => result.push_str(&format!(
        "start.push_attribute((\"{}\", self.{}));\n",
        f.attrs.value, f.name
      )),
      FieldType::Others(_) => result.push_str(&format!(
        "start.push_attribute((\"{}\", self.{}.as_str()));\n",
        f.attrs.value, f.name
      )),
    }

    if f.is_option {
      result.push_str("}\n");
    }
  }

  result.push_str(&format!("w.write_event(Event::{}(start))?;\n", event));

  if s.attrs.key == "parent" {
    for f in s.filter_field("child") {
      if f.is_option {
        result.push_str(&format!(
          "if let Some(ref __{0}) = self.{0} {{ __{0}.write(w); }}",
          f.name
        ));
      } else if f.is_vec {
        result.push_str(&format!(
          "for __{0} in &self.{0} {{ __{0}.write(w); }}",
          f.name
        ));
      } else {
        result.push_str(&format!("self.{}.write(w);\n", f.name));
      }
    }

    result.push_str(&format!(
      "w.write_event(Event::End(BytesEnd::borrowed(b\"{}\")))?;\n",
      s.attrs.value,
    ));
  } else if s.attrs.key == "text" {
    result.push_str(&format!(
      r#"w.write_event(Event::Text(BytesText::from_plain_str(self.{}.as_ref())))?;
         w.write_event(Event::End(BytesEnd::borrowed(b"{}")))?;"#,
      s.find_field("text").name,
      s.attrs.value,
    ));
  }

  result.push_str("Ok(())");

  result
}

pub(crate) fn impl_write_enum(e: &Enum) -> String {
  let mut result = String::with_capacity(1000);

  result.push_str(r#"match self {"#);

  for f in &e.fields {
    result.push_str(&format!("{}::{}(__p) => __p.write(w),", e.name, f.name));
  }

  result.push_str("}");

  result
}

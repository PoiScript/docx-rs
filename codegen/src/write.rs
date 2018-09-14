use types::{FieldType, Structure};

pub(crate) fn impl_write(structure: &Structure) -> String {
  let mut result = String::with_capacity(1000);

  let event = match structure.attrs.key.as_ref() {
    "parent" | "text" => "Start",
    "empty" => "Empty",
    _ => unreachable!("element should be one of the following types: parent, text or empty."),
  };

  result.push_str(&format!(
    r#"let mut start= BytesStart::borrowed(b"{0}", b"{0}".len());
"#,
    structure.attrs.value,
  ));

  for f in structure.filter_field("attr") {
    if f.is_option {
      result.push_str(&format!("if let Some(ref {0}) = self.{0} {{\n", f.name));
    }
    // TODO more types
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
      _ => (),
    }

    if f.is_option {
      result.push_str("}\n");
    }
  }

  result.push_str(&format!("writer.write_event(Event::{}(start))?;\n", event));

  if structure.attrs.key == "parent" {
    for f in structure.filter_field("child") {
      if f.is_option {
        result.push_str(&format!(
          "if let Some(ref __{0}) = self.{0} {{ __{0}.write(writer); }}",
          f.name
        ));
      } else if f.is_vec {
        result.push_str(&format!(
          "for __{0} in &self.{0} {{ __{0}.write(writer); }}",
          f.name
        ));
      } else {
        result.push_str(&format!("self.{}.write(writer);\n", f.name));
      }
    }

    result.push_str(&format!(
      "writer.write_event(Event::End(BytesEnd::borrowed(b\"{}\")))?;\n",
      structure.attrs.value,
    ));
  } else if structure.attrs.key == "text" {
    result.push_str(&format!(
      r#"writer.write_event(Event::Text(BytesText::from_plain_str(self.{}.as_ref())))?;
         writer.write_event(Event::End(BytesEnd::borrowed(b"{}")))?;"#,
      structure.find_field("text").name,
      structure.attrs.value,
    ));
  }

  result.push_str("Ok(())");

  result
}

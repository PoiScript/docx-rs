use types::{FieldType, Structure};

pub(crate) fn impl_write(structure: &Structure) -> String {
  let mut result = String::with_capacity(100);

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
    // TODO more types
    match f.get_ty() {
      FieldType::String => result.push_str(&format!(
        "start.push_attribute((\"{}\", self.{}.as_ref()));\n",
        f.attrs.value, f.name
      )),
      FieldType::Str | FieldType::Cow => result.push_str(&format!(
        "start.push_attribute((\"{}\", self.{}));\n",
        f.attrs.value, f.name
      )),
      _ => (),
    }
  }

  result.push_str(&format!("writer.write_event(Event::{}(start))?;\n", event));

  if structure.attrs.key == "parent" {
    for f in structure.filter_field("child") {
      result.push_str(&format!("self.{}.write(writer);\n", f.name));
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

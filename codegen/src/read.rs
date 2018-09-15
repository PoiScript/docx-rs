use types::{Enum, Struct};

pub(crate) fn impl_read_struct(s: &Struct) -> String {
  let mut result = String::with_capacity(1000);

  result.push_str(
    r#"let mut buf = Vec::new();
loop {
  match r.read_event(&mut buf) {"#,
  );

  if s.attrs.key == "parent" || s.attrs.key == "text" {
    result.push_str(&format!(
      r#"Ok(Event::Start(ref e)) => {{
           if e.name() == b"{}" {{
             return {}::read_with_bytes_start(e, r);
           }} else {{
             // TODO: throws an error
           }}
         }},
         Ok(Event::Empty(_)) => {{
           // TODO: throws an error
         }}"#,
      s.attrs.value, s.name
    ));
  } else if s.attrs.key == "empty" {
    result.push_str(&format!(
      r#"Ok(Event::Empty(ref e)) => {{
           if e.name() == b"{}" {{
             return {}::read_with_bytes_start(e, r);
           }} else {{
             // TODO: throws an error
           }}
         }}
         Ok(Event::Start(_)) => {{
           // TODO: throws an error
         }}"#,
      s.attrs.value, s.name
    ));
  }

  result.push_str(
    "Ok(Event::Eof) | Ok(Event::End(_)) => break,
    _ => (),
  };

  buf.clear();
}
unreachable!();",
  );

  result
}

pub(crate) fn impl_read_with_bytes_start_struct(s: &Struct) -> String {
  let mut result = String::with_capacity(1000);

  for f in s.filter_field("attr") {
    if f.is_vec {
      panic!("attributes can't be vector");
    }

    result.push_str(&format!("let mut __{} = None;", f.name));
  }

  result.push_str(
    r#"for attr in bs.attributes().filter_map(|a| a.ok()) {
  match attr.key {"#,
  );

  for f in s.filter_field("attr") {
    result.push_str(&format!(
      r#"b"{}" => __{} = Some(String::from_utf8(attr.value.into_owned().to_vec()).unwrap()),"#,
      f.attrs.value, f.name
    ));
  }

  result.push_str("_ => (), } }");

  if s.attrs.key == "parent" {
    for f in s.filter_field("child") {
      result.push_str(&format!(
        "let mut __{} = {};",
        f.name,
        if f.is_vec { "Vec::new()" } else { "None" }
      ));
    }

    result.push_str(
      r#"let mut buf = Vec::new();
loop {
  match r.read_event(&mut buf) {
    Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
      match e.name() {"#,
    );

    for f in s.filter_field("child") {
      if f.is_vec {
        result.push_str(&format!(
          r#"b"{}" => __{}.push({}::read_with_bytes_start(e, r)),"#,
          f.attrs.value, f.name, f.ty
        ));
      } else {
        result.push_str(&format!(
          r#"b"{}" => __{} = Some({}::read_with_bytes_start(e, r)),"#,
          f.attrs.value, f.name, f.ty
        ));
      }
    }

    result.push_str(&format!(
      r#"
        // TODO throws an error here, too
        _ => ()
      }}
    }}
    Ok(Event::Eof) => break,
    Ok(Event::End(ref e)) => {{
      if e.name() == b"{}" {{
        break;
      }} else {{
        // TODO: throws an error
      }}
    }}
    _ => (),
  }};

  buf.clear();
}}"#,
      s.attrs.value
    ));
  } else if s.attrs.key == "text" {
    result.push_str(&format!(
      r#"let mut __text = None;
  let mut buf = Vec::new();
  loop {{
    match r.read_event(&mut buf) {{
      Ok(Event::Text(e)) => {{
        __text = Some(String::from_utf8(e.escaped().to_vec()).unwrap());
      }}
      Ok(Event::Eof) => break,
      Ok(Event::End(ref e)) => {{
        if e.name() == b"{}" {{
          break;
        }} else {{
          // TODO: throws an error
        }}
      }}
      _ => (),
    }};

    buf.clear();
  }}"#,
      s.attrs.value
    ));
  }

  result.push_str(&format!("{} {{", s.name));

  for f in s.filter_field("attr") {
    if f.is_option {
      result.push_str(&format!(r#"{0}: __{0},"#, f.name));
    } else {
      result.push_str(&format!(r#"{0}: __{0}.expect("bla"),"#, f.name));
    }
  }

  if s.attrs.key == "parent" {
    for f in s.filter_field("child") {
      if f.is_option || f.is_vec {
        result.push_str(&format!(r#"{0}: __{0},"#, f.name));
      } else {
        result.push_str(&format!(r#"{0}: __{0}.expect("bla"),"#, f.name));
      }
    }
  } else if s.attrs.key == "text" {
    result.push_str(&format!(
      r#"{}: __text.expect("bla"),"#,
      s.find_field("text").name
    ));
  }

  result.push_str("}");

  result
}

pub(crate) fn impl_read_enum(e: &Enum) -> String {
  let mut result = String::with_capacity(100);

  result.push_str(r#"match std::str::from_utf8(bs.name()).unwrap() {"#);

  for f in &e.fields {
    result.push_str(&format!(
      r#""{}" => {}::{}({}::read_with_bytes_start(bs, r)),"#,
      f.attrs.value, e.name, f.name, f.ty
    ));
  }

  result.push_str(
    r#"_ => panic!("bla")  // TODO throws an error
}"#,
  );

  result
}

pub(crate) fn impl_read_with_bytes_start_enum(e: &Enum) -> String {
  let mut result = String::with_capacity(100);

  result.push_str(
    r#"let mut buf = Vec::new();
loop {
  match r.read_event(&mut buf) {
    Ok(Event::Start(ref e)) => {
      match e.name() {"#,
  );

  for f in e.filter_field("text") {
    result.push_str(&format!(
      r#"b"{}" => return Self::read_with_bytes_start(e, r),"#,
      f.attrs.value
    ));
  }

  for f in e.filter_field("child") {
    result.push_str(&format!(
      r#"b"{}" => return Self::read_with_bytes_start(e, r),"#,
      f.attrs.value
    ));
  }

  result.push_str(
    "_ => unreachable!(), // TODO: throws an error
  }
},
Ok(Event::Empty(ref e)) => {
      match e.name() {",
  );

  for f in e.filter_field("empty") {
    result.push_str(&format!(
      r#"b"{}" => return Self::read_with_bytes_start(e, r),"#,
      f.attrs.value
    ));
  }

  result.push_str(
    "_ => unreachable!(), // TODO: throws an error
      }
    },
    _ => unreachable!(), // TODO: throws an error
  }
}
unreachable!();",
  );

  result
}

use quick_xml::events::Event;
use quick_xml::Result;
use quick_xml::Writer;
use std::collections::LinkedList;
use std::io::Cursor;

pub trait Element<'a> {
  fn default() -> Self;

  fn events(&self) -> LinkedList<Event<'a>>;

  fn write(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
    for event in self.events() {
      writer.write_event(event)?;
    }
    Ok(())
  }

  fn generate(&self) -> Vec<u8> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    self.write(&mut writer);

    writer.into_inner().into_inner()
  }
}

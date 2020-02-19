use docx::formatting::TableProperty;
use docx::{document::Table, document::TableGrid, document::TableRow, Docx, Result};

fn main() -> Result<()> {
    // create an empty document
    let mut docx = Docx::default();

    // create a table and populate it with data
    let tbl = Table::default()
        .prop(TableProperty::default())
        .push_grid(vec![1, 2, 3])
        .push_grid(TableGrid::default())
        .push_row(TableRow::default());
    // add the table to the document
    docx.document.push(tbl);
    // persist the document to a file
    docx.write_file("table.docx")?;

    Ok(())
}

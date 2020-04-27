use docx::{
    document::{Paragraph, Table, TableCell, TableGrid, TableRow},
    formatting::{TableCellProperty, TableProperty, TableRowProperty},
    Docx, DocxResult,
};

fn main() -> DocxResult<()> {
    // Create an empty document
    let mut docx = Docx::default();

    // Create a table and populate it with data
    let tbl = Table::default()
        .property(TableProperty::default())
        .push_grid(vec![1, 2, 3])
        .push_grid(TableGrid::default())
        .push_row(
            TableRow::default()
                .property(TableRowProperty::default())
                .push_cell(Paragraph::default())
                .push_cell(
                    TableCell::pargraph(Paragraph::default())
                        .property(TableCellProperty::default()),
                ),
        );

    // Add the table to the document
    docx.document.push(tbl);

    // Persist the document to a file
    docx.write_file("table.docx")?;

    Ok(())
}

use tabular::Table;
use tabular::TableFormat;

fn main() {
    let table = Table {
        names  : ["hello".to_owned(), "world".to_owned()],
        values : vec![
            ["42".to_owned(),                                         "a very very long string".to_owned()],
            ["Extremely long string put here for testing".to_owned(), "dummy".to_owned()],
            ["中文".to_owned(), "dummy".to_owned()],
        ],
    };

    println!("Pretty:"); table.print(TableFormat::Pretty);
    println!("Json:");   table.print(TableFormat::Json);
    println!("CSV:");    table.print(TableFormat::CSV);
}

use tabular::Table;

fn main() {
    let table = Table {
        names  : ["hello".to_owned(), "world".to_owned()],
        values : vec![
            ["42".to_owned(),                                         "a very very long string".to_owned()],
            ["Extremely long string put here for testing".to_owned(), "dummy".to_owned()],
        ],
    };

    println!("Pretty:"); table.print_pretty();
    println!("Json:");   table.print_json();
    println!("CSV:");    table.print_csv();
}

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(array_methods)]

pub struct Table<const N: usize> {
    pub names  : [String; N],
    pub values : Vec<[String; N]>,
}

pub enum TableFormat {
    Pretty,
    Json,
    CSV,
}

impl<const N: usize> Table<N> {
    fn print_pretty(&self) {
        let column_widths = std::iter::empty()
            .chain(std::iter::once(&self.names))
            .chain(self.values.iter())
            .map(|rows| rows.each_ref().map(|entry| unicode_width::UnicodeWidthStr::width(entry.as_str())))
            .fold([0; N], |widths1, widths2| {
                let mut widths = [0; N];
                for i in 0..N {
                    widths[i] = std::cmp::max(widths1[i], widths2[i]);
                }
                widths
            });

        let table_width = column_widths.iter().sum::<usize>() + 3 * N + 1;

        let print_separator = || {
            for _ in 0..table_width {
                print!("=");
            }
            println!("");
        };

        let print_row = |row : &[String; N]| {
            for (column_width, entry) in std::iter::zip(column_widths, row) {
                let entry_width = unicode_width::UnicodeWidthStr::width(entry.as_str());
                print!("| ");
                print!("{entry}");
                for _ in entry_width..column_width {
                    print!(" ");
                }
                print!(" ");
            }
            println!("|");
        };

        print_separator();
        print_row(&self.names);
        print_separator();
        self.values.iter().for_each(print_row);
        print_separator();
    }

    fn print_json(&self) {
        print!("[");
        let mut outer_sep = "";
        for values in &self.values {
            print!("{outer_sep}");
            print!("{{");
            let mut inner_sep = "";
            for (name, value) in std::iter::zip(&self.names, values) {
                print!("{inner_sep}");
                print!("\"{name}\":\"{value}\"");
                inner_sep = ",";
            }
            print!("}}");
            outer_sep = ",";
        }
        print!("]");
        println!("");
    }

    fn print_csv(&self) {
        let print_row = |row| {
            let mut first = true;
            for entry in row {
                if first {
                    first = false;
                } else {
                    print!(",");
                }
                print!("{entry}");
            }
            println!("");
        };

        print_row(&self.names);
        self.values.iter().for_each(print_row);
    }

    pub fn print(&self, format : TableFormat) {
        match format {
            TableFormat::Pretty => self.print_pretty(),
            TableFormat::Json   => self.print_json(),
            TableFormat::CSV    => self.print_csv(),
        }
    }
}


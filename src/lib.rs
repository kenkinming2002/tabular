#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(array_methods)]

pub struct Table<const N: usize> {
    pub names  : [String; N],
    pub values : Vec<[String; N]>,
}

impl<const N: usize> Table<N> {
    pub fn print_pretty(&self) {
        let widths = std::iter::empty()
            .chain(std::iter::once(&self.names))
            .chain(self.values.iter())
            .map(|rows| rows.each_ref().map(|entry| entry.len()))
            .fold([0; N], |widths1, widths2| {
                let mut widths = [0; N];
                for i in 0..N {
                    widths[i] = std::cmp::max(widths1[i], widths2[i]);
                }
                widths
            });

        let width = widths.iter().sum::<usize>() + 3 * N + 1;

        let print_separator = || {
            for _ in 0..width {
                print!("=");
            }
            println!("");
        };

        let print_row = |row| {
            for (width, entry) in std::iter::zip(widths, row) {
                // FIXME: Unicode Width
                print!("| ");
                print!("{entry:width$}");
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

    pub fn print_json(&self) {
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

    pub fn print_csv(&self) {
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
}


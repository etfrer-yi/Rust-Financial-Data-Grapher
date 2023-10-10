extern crate csv;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let csv_file_path = "./all_stocks_5yr.csv";
    let column_index = 6;
    let file = File::open(csv_file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut unique_strings = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(column_value) = record.get(column_index) {
            unique_strings.insert(column_value.to_owned());
        }
    }

    for unique_string in &unique_strings {
        println!("{}", unique_string);
    }

    Ok(())
}

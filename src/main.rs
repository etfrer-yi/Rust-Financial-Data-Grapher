extern crate csv;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

fn extract_unique_column_values(csv_file_path: &str, column_index: usize) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(csv_file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut unique_strings = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(column_value) = record.get(column_index) {
            unique_strings.insert(column_value.to_owned());
        }
    }

    Ok(unique_strings)
}

fn main() -> Result<(), Box<dyn Error>> {
    let csv_file_path = "./all_stocks_5yr.csv";
    let column_index = 6;

    if let Ok(unique_values) = extract_unique_column_values(csv_file_path, column_index) {
        for value in unique_values {
            println!("{}", value);
        }
    }

    Ok(())
}

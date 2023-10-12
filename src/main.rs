
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;

extern crate plotly;
use plotly::common::{TickFormatStop, Title};
use plotly::layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode};
use plotly::{Candlestick, Layout, Ohlc, Plot, Scatter};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

fn extract_unique_column_values() -> Result<HashSet<String>, Box<dyn Error>> {
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

    Ok(unique_strings)
}

fn get_row_data() {
    let csv_file_path = "./all_stocks_5yr.csv";
    let column_index = 6;
    let file = File::open(csv_file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        if let Some(column_value) = record.get(column_index) {
            if column_value == search_value {
                println!("{:?}", record);
            }
        }
    }
}

fn main() {
    // if let Ok(unique_values) = extract_unique_column_values() {
    //     println!("Enter your desired column among those listed below :");
    //     println!("{:?}", unique_values);
    //     let mut line = String::new();
    //     io::stdin().read_line(&mut line).expect("Failed to read line");
    //     let trimmed_line = line.trim();
    //     get_row_data();
    // }
    time_series_plot_with_custom_date_range(true);
}


fn time_series_plot_with_custom_date_range(show: bool) {
    let data = load_apple_data();
    let date = data.iter().map(|d| d.date.clone()).collect();
    let high = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range(vec!["2016-07-01", "2016-12-31"]))
        .title(Title::new("Manually Set Date Range"));
    plot.set_layout(layout);

    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("time_series_plot_with_custom_date_range"))
    );
}

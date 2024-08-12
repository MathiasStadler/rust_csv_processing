// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
// use csv::Reader;
use serde::Deserialize;

// https://github.com/BurntSushi/rust-csv/issues/125

// Date,Open,High,Low,Close,Volume

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Open")]
    open: f32,

    #[serde(rename = "High")]
    high: f32,

    #[serde(rename = "Low")]
    low: f32,

    #[serde(rename = "Close")]
    close: f32,

    #[serde(rename = "Volume")]
    volume: f32,
}

fn main() {
    let mut reader = csv::Reader::from_path("data/trex_us_d.csv").unwrap();
    // let mut reader = csv::Reader::from_path("data/car.csv").unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);

    // https://stackoverflow.com/questions/75287355/reading-csv-file-when-header-is-present-with-lower-or-upper-case

    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        println!(
            " {:?}, {:?}, {:?}, {:?},{:?}, {:?}",
            record.date, record.open, record.high, record.low, record.close, record.volume,
        );
    }
}

// cargo run --example csv_read_4

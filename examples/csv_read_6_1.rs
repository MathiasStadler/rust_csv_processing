// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
use csv::Reader;
use serde::Deserialize;

// Date,Open,High,Low,Close,Volume
#[derive(Deserialize)]
struct Record {
    #[allow(non_snake_case)]
    Date: String,
    #[allow(non_snake_case)]
    Open: f16,
    #[allow(non_snake_case)]
    High: f16,
    #[allow(non_snake_case)]
    Low: f16,
    #[allow(non_snake_case)]
    Close: f16,
    #[allow(non_snake_case)]
    Volume: f16,
}

fn main() {
    let mut reader = csv::Reader::from_path("data/trex_us_d.csv").unwrap();
    // let mut reader = csv::Reader::from_path("data/car.csv").unwrap();
    // for result in rdr.records() {
    //     let record = result.unwrap();
    //     println!("{:?}", record);

    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        println!(
            " {}, {}, {}, {},{}, {}",
            record.Date,
            record.Open,
            record.High,
            record.Low,
            record.Close,
            record.Volume,
        );
    }
}

// cargo run --example csv_read_4

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
    #[allow(non_snake_case)]
    #[serde(rename = "Date")]
    date: String,
    #[allow(non_snake_case)]
    Open: f32,
    #[allow(non_snake_case)]
    High: f32,
    #[allow(non_snake_case)]
    Low: f32,
    #[allow(non_snake_case)]
    Close: f32,
    #[allow(non_snake_case)]
    Volume: f32,
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
            record.date,
            record.open,
            record.High,
            record.Low,
            record.Close,
            record.Volume,
        );
    }
}

// cargo run --example csv_read_4

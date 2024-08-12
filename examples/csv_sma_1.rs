// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
// use csv::Reader;

use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;

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

    let mut sma = Sma::new(7).unwrap();
    let mut reader = csv::Reader::from_path("data/trex_us_d.csv").unwrap();

    // add sma

    // for record in reader.deserialize() {

    //     let record: Record = record.unwrap();

    //     println!(
    //         " {:?}, {:?}, {:?}, {:?},{:?}, {:?}",
    //         record.date, record.open, record.high, record.low, record.close, record.volume,
    //     );
    // }

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();

        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        
        let sma_val = sma.next(&dt);
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
        println!(
                     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
                     date,open, high,low, close,volume,sma_val);
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
    }

}

// cargo run --example csv_sma_1

// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
// use csv::Reader;
use csv::Writer;

use ta::indicators::SimpleMovingAverage as Sma;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
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

    // macd 12,26,9
    let mut sma = Sma::new(7).unwrap();
    let mut macd = Macd::new(12,26,9).unwrap();

    let mut reader = csv::Reader::from_path("data/trex_us_d.csv").unwrap();
    // FROM HERE - https://docs.rs/csv/latest/csv/struct.Writer.html
    let mut wtr = Writer::from_path("output.csv").unwrap();

    // add sma

    // for record in reader.deserialize() {

    //     let record: Record = record.unwrap();

    //     println!(
    //         " {:?}, {:?}, {:?}, {:?},{:?}, {:?}",
    //         record.date, record.open, record.high, record.low, record.close, record.volume,
    //     );
    // }

    wtr.write_record(&["Date", "Open", "High", "Low", "Close", "Volume", "SMA(7)"])
        .unwrap();
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
        let macd_val:f64 = macd.next(&dt);
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
        println!(
            " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}, {:2.2}",
            date, open, high, low, close, volume, sma_val,macd_val
        );
        wtr.write_record(&[
            date,
            open.to_string(),
            high.to_string(),
            low.to_string(),
            close.to_string(),
            volume.to_string(),
            sma_val.to_string(),
        ])
        .unwrap();
        // wtr.write_record(
        //     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
        //     date, open, high, low, close, volume, sma_val
        // );
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
    }
}

// cargo run --example csv_indicator_1

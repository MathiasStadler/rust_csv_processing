// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
// use csv::Reader;
use csv::Writer;

use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;

fn main() {
    // vec of tuple

    let pair: (i32, String, bool) = (1, String::from("Hallo"), true);
    println!("Pair is {:?}", pair);

    let pair: (&'static str, f64, f64, f64) = ("Hallo", 1.0, 2.0, 3.0);
    println!("str is {:?}", pair);

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut t1: (&'static str, f32, f32, f32, f32) = ("2019-04-25", 100.10, 200.20, 300.30, 400.40);

    // tuple aces one value

    println!("tuples v0 => {}", t1.0);
    println!("tuples v1 => {}", t1.1);
    println!("tuples v2 => {}", t1.2);
    println!("tuples v3 => {}", t1.3);
    println!("tuples v4 => {}", t1.4);

    // tuple change values
    t1.1= 69.15;
    println!("changes tuples v1 => {}", t1.1);

    // print values again
    println!("tuples v0 => {}", t1.0);
    println!("tuples v1 => {}", t1.1);
    println!("tuples v2 => {}", t1.2);
    println!("tuples v3 => {}", t1.3);
    println!("tuples v4 => {}", t1.4);



    // add value to tuple
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut t2: () = ();




    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut stock_data_vec: Vec<(&'static str, f32, f32, f32, f32)> = Vec::new();

    // add t to stock_data_vec
    stock_data_vec.push(t1);

    // print vec

    println!("{:?}",stock_data_vec);

    // https://www.schwab.com/learn/story/understanding-simple-moving-average-crossovers
    // 10 50 200
    let mut sma_7 = Sma::new(7).unwrap();
    let mut sma_10 = Sma::new(10).unwrap();
    let mut sma_50 = Sma::new(50).unwrap();
    let mut sma_200 = Sma::new(200).unwrap();
    let mut reader = csv::Reader::from_path("data/trex_us_d.csv").unwrap();
    // FROM HERE - https://docs.rs/csv/latest/csv/struct.Writer.html
    let mut wtr = Writer::from_path("output_sma_9.csv").unwrap();

    wtr.write_record(&[
        "Date", "Open", "High", "Low", "Close", "Volume", "SMA(7)", "SMA(10)", "SMA(50)",
        "SMA(200)",
    ])
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

        let sma_7_val = sma_7.next(&dt);
        let sma_10_val = sma_10.next(&dt);
        let sma_50_val = sma_50.next(&dt);
        let sma_200_val = sma_200.next(&dt);

        // println!("{}: {} = {:2.2}", date, sma_7, sma_7_val

        // println!(
        //     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
        //     date, open, high, low, close, volume, sma_7_val

        // );
        wtr.write_record(&[
            date,
            open.to_string(),
            high.to_string(),
            low.to_string(),
            close.to_string(),
            volume.to_string(),
            sma_7_val.to_string(),
            sma_10_val.to_string(),
            sma_50_val.to_string(),
            sma_200_val.to_string(),
        ])
        .unwrap();
    }
}

// cargo run --example csv_sma_5

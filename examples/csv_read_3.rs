// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// Read CSV records with different delimiter


// cargo add serde_json

use csv::Error;
use serde::Deserialize;




#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

use csv::ReaderBuilder;

fn main() -> Result<(), Error> {
    let data = "name\tplace\tid
        Mark\tMelbourne\t46
        Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }

    Ok(())
}

// cargo run --example csv_read_3
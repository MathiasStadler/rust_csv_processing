// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// sample for explain read from file system

use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn main() -> Result<(), csv::Error> {
    let _csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";


    // let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut reader = csv::Reader::from_path("data/car.csv").unwrap();



    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description
        );
    }

    Ok(())
}

// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// Serialize records to CSV

use std::io;

fn main()  {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["Name", "Place", "ID"]).unwrap();

    wtr.serialize(("Mark", "Sydney", 87)).unwrap();
    wtr.serialize(("Ashley", "Dublin", 32)).unwrap();
    wtr.serialize(("Akshat", "Delhi", 11)).unwrap();

    wtr.flush().unwrap();
    
}


// cargo run --example csv_read_4
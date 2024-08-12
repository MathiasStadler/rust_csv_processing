// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// Serialize records to CSV using Serde

// use std::fmt::{Debug, Formatter};
use std::fmt;
use serde::Serialize;
use std::io;

#[derive(Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

//added
impl  std::fmt::Debug  for Record<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Record {{ name: {}, place: {}, id; {} }}", self.name, self.place,self.id )
    }
}

// FROM HERE
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/fmt/trait.Debug.html

// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }


// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
//     }
// }



fn main() -> Result<(),Box<Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    //Ok(())
    Ok::<(),  Eq>(())
}

// cargo run --example csv_read_5



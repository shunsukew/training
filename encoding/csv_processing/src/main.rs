extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use csv::{Error, ReaderBuilder, Reader, Writer};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;
use std::io;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Record2 {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

#[derive(Serialize)]
struct Record3<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
    1948,Porsche,356,Luxury sports car
    1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

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

    // read csv with a tab delimitter
    let data = "name\tplace\tid
    Mark\tMelbourne\t46
    Ashley\tZurich\t92";

    let mut reader2 = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
    for result in reader2.deserialize::<Record2>() {
        println!("{:?}", result?);
    }

    // serialize data to csv
    let mut wtr = Writer::from_writer(io::stdout());
    wtr.write_record(&["Name", "Place", "ID"])?;
    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;

    // serialize records to csv useing Serde
    let mut wtr2 = Writer::from_writer(io::stdout());
    let rec1 = Record3 { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record3 { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record3 { name: "Akshat", place: "Delhi", id: 98};

    wtr2.serialize(rec1)?;
    wtr2.serialize(rec2)?;
    wtr2.serialize(rec3)?;

    wtr2.flush()?;

    Ok(())
}
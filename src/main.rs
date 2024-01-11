extern crate csv;
extern crate serde;
mod write;

use std::{collections::HashMap, env, error::Error, ffi::OsString, fs::File, io, process};

use serde_derive::Deserialize;
use write::runWrite;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}

fn main() {
    if let Err(err) = runWrite() {
        println!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_file_path() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        Some(file_path) => Ok(file_path),
        None => Err(From::from("Expected 1 argument, but got none.")),
    }
}

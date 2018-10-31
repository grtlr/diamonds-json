extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Diamond {
    carat: f32,
    cut: String,
    color: String,
    clarity: String,
    depth: f32,
    table: f32,
    price: f32,
    x: f32,
    y: f32,
    z: f32,
}

fn read_csv(f: File) -> Vec<Diamond> {
    let mut rdr = csv::Reader::from_reader(f);
    rdr.deserialize().filter_map(Result::ok).collect()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let infile = File::open(&args[1])?;
            let diamonds = read_csv(infile);
            println!("{}", serde_json::to_string(&diamonds).unwrap());
            Ok(())
        }
        _ => {
            println!("usage: diamonds-json <input-csv>");
            Ok(())
        }
    }
}

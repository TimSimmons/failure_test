extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use failure::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    m: HashMap<(i32, i32), bool>,
}

fn main() {
    println!("Hello, world!");
    let path = "example.txt";
    match open(path) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }

    let mut m: HashMap<(i32, i32), bool> = HashMap::new();

    m.insert((1, 1), true);
    let point = Point { x: 1, y: 2, m: m };
    match point_to_json(point) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("error: {}, {}", e.as_fail(), e.backtrace()),
    }
}

fn open(path: &str) -> Result<String, Error> {
    let mut string = String::new();
    File::open(path)?.read_to_string(&mut string)?;
    Ok(string)
}

fn point_to_json(point: Point) -> Result<String, Error> {
    let s = serde_json::to_string(&point)?;
    Ok(s)
}

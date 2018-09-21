extern crate failure;

use std::fs::File;
use std::io::prelude::*;

use failure::Error;

fn main() {
    println!("Hello, world!");
    let path = "example.txt";
    match open(path) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }
}

fn open(path: &str) -> Result<String, Error> {
    let mut string = String::new();
    File::open(path)?.read_to_string(&mut string)?;
    Ok(string)
}

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct PyFunction {
    ast_type: String,
    body: Vec<PyBody>
}

#[derive(Serialize, Deserialize, Debug)]
struct PyBody {
    col_offset: u8,
    lineno: u8
}


fn read_code_from_file<P: AsRef<Path>>(path: P) -> Result<PyFunction, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(file)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let u = read_code_from_file("../client/file1.json").unwrap();
    println!("{:#?}", u);
}
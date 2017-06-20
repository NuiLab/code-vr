extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct PyFunction {
    name: String
    /*lineno: u32,
    ast_type: String,
    args: u32,*/
}

#[cfg(test)]
mod test {

    use ast::function::PyFunction;
    use serde_json::from_str;

    fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
        ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
    }

    #[test]
    fn distance_test() {
        assert!(distance((0f32, 0f32), (1f32, 1f32)) == (2f32).sqrt());
    }

    fn read_code_from_file<P: AsRef<Path>>(path: P) -> Result<PyFunction, Box<Error>>
    {
        // Open the file in read-only mode.
        let file = File::open(path)?;

        // Read the JSON contents of the file as an instance of `PyFunction`.
        let u = serde_json::from_reader(file)?;

        // Return the `PyFunction`.
        Ok(u)
    }

    #[test]
    fn convert_test() {
        println!("attempting to parse with serde");
        let pf = read_code_from_file("../../file1.json").unwrap();
        println!("{:?}", pf);
    }


}


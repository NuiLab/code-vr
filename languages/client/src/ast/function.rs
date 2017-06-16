use std::fs::File;
use std::io::prelude::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct PyFunction {
    name: String,
    lineno: u32,
    ast_type: String,
    args: u32,
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

    pub fn convert(json_string: String) -> PyFunction {
        let ast: PyFunction = from_str(json_string.as_str()).unwrap();
        ast
    }

    #[test]
    fn convert_test() {
        println!("hello from test");
    }


}


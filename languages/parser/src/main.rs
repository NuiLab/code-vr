#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct PyAssign
{
    ast_type: String,
    col_offset: u8,
    lineno: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct PyFunctionDef
{
    //args
    ast_type: String,
    //body
    col_offset: u8,
    //decorator_list
    lineno: u8,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PyExpr
{
    ast_type: String,
    col_offset: u8,
    lineno: u8,
    //value
}

#[derive(Serialize, Deserialize, Debug)]
struct PyBody {
    ast_type: String,
    col_offset: u8,
    lineno: u8,
    //targets: Vec<PyTarget>
}

#[derive(Serialize, Deserialize, Debug)]
struct PyFunction {
    ast_type: String,
    body: Vec<serde_json::Value>
}

fn read_code_from_file<P: AsRef<Path>>(path: P) -> Result<PyFunction, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let first_pass = serde_json::from_reader(file)?;

    // Return the `User`.
    Ok(first_pass)
}

fn main() {
    let nodes = read_code_from_file("../client/file1.json").unwrap();
    println!("Parsed {:?} nodes:\n", nodes.body.len());
    
    //println!("{:#?}", nodes.body[0]);

    //parse each node
    for i in 0..nodes.body.len()
    {
        //parse each node
        let n: PyBody = serde_json::from_value(nodes.body[i].clone()).unwrap();
        match n.ast_type.as_ref() {
            "Assign" => {
                //println!("parsing Assign node:");
                let assign: PyAssign = serde_json::from_value(nodes.body[i].clone()).unwrap();
                println!("{:?}", assign.ast_type);
            },
            "FunctionDef" => {
                //println!("parsing FunctionDef node:");
                let function_def: PyFunctionDef = serde_json::from_value(nodes.body[i].clone()).unwrap();
                println!("{:?}, {:?}", function_def.ast_type, function_def.name);
            },
            "Expr" => { 
                //println!("parsing Expr node:");
                let expr: PyExpr = serde_json::from_value(nodes.body[i].clone()).unwrap();
                println!("{:?}", expr.ast_type);
            },
            _ => println!("matched unknown node, no action taken: {:?}", n.ast_type)
        }
        //println!("{:?}", n);
    }
}
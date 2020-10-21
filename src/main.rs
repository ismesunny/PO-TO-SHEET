
use std::process::Command;
use std::process::{Stdio};
use std::fs::File;
use std::env;

fn main(){
    // let import_name = "konsole.po";
    // let export_name = "test.csv";

    let import_name = "file/original_file/konsole.po";
    let export_name = "file/converted_file/test.csv";

    let program = "node";
   
    let outputs = File::create(export_name).unwrap();
    let errors = outputs.try_clone().unwrap();
    let status =
     
    Command::new(program)
    .args(&[format!("{}/index.js",env::current_dir().unwrap().display()),
    format!("{}/{}",env::current_dir().unwrap().display(),import_name)])
    .stdout(Stdio::from(outputs))
    .stderr(Stdio::from(errors))
    .spawn();

    println!("status : {:#?}", status);

}


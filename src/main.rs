
use std::process::Command;
use std::process::{Stdio};
use std::fs::File;
use std::env;

fn main(){

    let import_name = "name file in /data/import"; //example /data/import/name.po
    let export_name = "name file in data/export";  //example /data/export/name.csv

    let program = "node";
   
    let outputs = File::create(export_name).unwrap();
    let errors = outputs.try_clone().unwrap();
    let _status =
     
    Command::new(program)
    .args(&[format!("{}/index.js",env::current_dir().unwrap().display()),
    format!("{}/{}",env::current_dir().unwrap().display(),import_name)])
    .stdout(Stdio::from(outputs))
    .stderr(Stdio::from(errors))
    .spawn();

    //println!("status : {:#?}", status);
}


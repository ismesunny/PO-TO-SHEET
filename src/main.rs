
use std::process::Command;
use std::process::{Stdio};
use std::fs::File;
use std::env;

fn main(){

    let import_name = "import filelocation or name";
    let export_name = "export filelocation or name";

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

    //println!("status : {:#?}", status);
}


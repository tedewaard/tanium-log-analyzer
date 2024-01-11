use std::{path::Path, fs};


const BASE_PATH: &str = "C:/Program Files (x86)/Tanium/Tanium Client";


fn list_files () {
    let path = Path::new(BASE_PATH); 
    for file in fs::read_dir(path).unwrap() {
        println!("{}", file.unwrap().path().display());
    };
}

fn main() {
    println!("Hello, world!");
    list_files();
}

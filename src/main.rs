use std::{path::Path, io::stdin, io::{stdout, Stdout, Write}, fs};


const BASE_PATH: &str = "C:/Program Files (x86)/Tanium/Tanium Client";
const SOFTWARE_MANAGEMENT_LOG: &str = "C:/Program Files (x86)/Tanium/Tanium Client/Tools/SoftwareManagement/logs/software-management.log";


fn list_files() {
    let path = Path::new(BASE_PATH); 
    for file in fs::read_dir(path).unwrap() {
        println!("{}", file.unwrap().path().display());
    };
}

fn get_input(prompt: &str) -> String {
    let close_prompt = " Use ctrl-c to quit";
    let new_prompt = prompt.to_owned() + close_prompt;
    let mut answer = String::new();
    println!("{}", new_prompt);
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut answer);

    return answer;
}

fn read_log_lines(path: &Path) {
    let lines = std::fs::read_to_string(path).unwrap_or("Error reading file.".to_string());
    for line in lines.lines() {
        println!("{}", line);
    }
}

fn main() {
    //println!("Hello, world!");
    //list_files();
    let p = "This is a test.";
    let input = get_input(p);
    println!("{}", input);
    let path = Path::new(SOFTWARE_MANAGEMENT_LOG);
    read_log_lines(path);
}

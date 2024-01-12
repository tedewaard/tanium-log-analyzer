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
    let close_prompt = " Use ctrl-c to quit: ";
    let new_prompt = prompt.to_owned() + close_prompt;
    let mut answer = String::new();
    print!("{}", new_prompt);
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut answer);

    return answer;
}

fn read_log_lines(path: &Path) -> String {
    let lines = std::fs::read_to_string(path).unwrap_or("Error reading file.".to_string());
    /*
    for line in lines.lines() {
        println!("{}", line);
    }
    */
    return lines
}

fn search_logs_for_deploy_job(lines: String, job_id: &str)  -> Vec<String>{
    let mut filtered_lines: Vec<String> = Vec::new();
    for line in lines.lines() {
        //println!("{}", line);
        if line.contains(job_id) {
            filtered_lines.push(line.to_string());
            println!("{}", line);
        }
    }
    return filtered_lines;
}

fn main() {
    //list_files();
    let p = "Enter the deploy job ID.";
    let input = get_input(p);
    //println!("{}", input);
    let path = Path::new(SOFTWARE_MANAGEMENT_LOG);
    let lines = read_log_lines(path);
    println!("{}", "Is it getting here?");
    let filtered = search_logs_for_deploy_job(lines, &input.trim()); 
    println!("{}", filtered.len());
}

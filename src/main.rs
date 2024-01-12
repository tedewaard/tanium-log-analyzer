use std::{path::Path, io::stdin, io::{stdout, Stdout, Write}, fs, collections::HashSet};
use regex::Regex;


const BASE_PATH: &str = "C:/Program Files (x86)/Tanium/Tanium Client";
const SOFTWARE_MANAGEMENT_LOG: &str = "C:/Program Files (x86)/Tanium/Tanium Client/Tools/SoftwareManagement/logs/software-management.log";
const TEST_LOG: &str = "C:/Program Files (x86)/Tanium/Tanium Client/Tools/SoftwareManagement/logs/software-management.log.1";

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

fn get_selfservice_install_ids(lines: String) -> Vec<usize>{
    let mut set = HashSet::new();
    let mut filtered_lines: Vec<usize> = Vec::new();
    let re = Regex::new(r"EUSS Deploy (\d+)\b").unwrap();
    for line in lines.lines() {
        //println!("{}", line);
        if line.contains("EUSS Deploy") {
            let num = re.captures(line);
            if num.is_some() {
                //let id_string = num.unwrap()[1].to_string();
                //println!("{}", id_string);
                let id = num.unwrap()[1].trim().parse::<usize>().unwrap();
                set.insert(id);
            }
        }
    }
    for id in set {
        filtered_lines.push(id);
    }
    return filtered_lines;
}

fn main() {
    //list_files();
    let p = "Enter the deploy job ID.";
    //let input = get_input(p);
    //println!("{}", input);
    let path = Path::new(TEST_LOG);
    let lines = read_log_lines(path);
    //let filtered = search_logs_for_deploy_job(lines, &input.trim()); 
    //println!("{}", filtered.len());
    let self_service_jobs = get_selfservice_install_ids(lines);
    println!("{:?}", self_service_jobs);

}

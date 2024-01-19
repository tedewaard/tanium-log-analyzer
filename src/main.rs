use std::{path::Path, io::stdin, io::{stdout, Write}, fs, collections::HashSet};
use regex::Regex;


const BASE_PATH: &str = "C:/Program Files (x86)/Tanium/Tanium Client";
const SOFTWARE_MANAGEMENT_LOG: &str = "C:/Program Files (x86)/Tanium/Tanium Client/Tools/SoftwareManagement/logs/software-management.log";
const TEST_LOG: &str = "C:/Program Files (x86)/Tanium/Tanium Client/Tools/SoftwareManagement/logs/software-management.log.1";

enum JobType {
    SelfService,
    Deploy
}

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

fn search_logs_for_job_id(lines: &String, id: &str, id_list: &Vec<(usize, &JobType)>)  -> Vec<String>{
    let mut filtered_lines: Vec<String> = Vec::new();
    let mut job_type: &JobType = &JobType::SelfService;
    for i in id_list {
        if i.0.to_string() == id {
           job_type = i.1; 
        }
    }
    let search_string = match job_type {
        JobType::SelfService => "EUSS Deploy ".to_owned() + id,
        JobType::Deploy => "Deploy ".to_owned() + id
    };
    for line in lines.lines() {
        if line.contains(&search_string) {
            filtered_lines.push(line.to_string());
            println!("{}", line);
        }
    }
    return filtered_lines;
}

//TODO: Return tuple with ID and SS Package Name
fn get_selfservice_install_ids(lines: &String, id_list: &mut Vec<(usize, &JobType)>) -> Vec<(usize, String)>{
    let mut set = HashSet::new();
    let mut ss_ids: Vec<(usize, String)> = Vec::new();
    //let re = Regex::new(r"EUSS Deploy (\d+)\b").unwrap();
    let re = Regex::new(r"\[EUSS Deploy (\d+) \((.+?)\)\]").unwrap();
    let filtered_lines = lines.lines().filter(|x| x.contains("EUSS Deploy")).collect::<Vec<&str>>();
    for line in filtered_lines {
        //println!("{}", line);
        let num = re.captures(line);
        if num.is_some() {
            let id = num.as_ref().unwrap()[1].trim().parse::<usize>().unwrap();
            let name = num.unwrap()[2].trim().to_string();
            set.insert((id, name));
        }
    }
    for id in set {
        ss_ids.push(id.clone());
        id_list.push((id.0, &JobType::SelfService));
    }
    return ss_ids;
}

fn get_deploy_install_ids(lines: &String, id_list: &mut Vec<(usize, &JobType)>) -> Vec<(usize, String)>{
    let mut set = HashSet::new();
    let mut ss_ids: Vec<(usize, String)> = Vec::new();
    //let re = Regex::new(r"EUSS Deploy (\d+)\b").unwrap();
    let re = Regex::new(r"\[Deploy (\d+) \((.+?)\)\]").unwrap();
    let filtered_lines = lines.lines().filter(|x| x.contains("[Deploy")).collect::<Vec<&str>>();
    for line in filtered_lines {
        //println!("{}", line);
        let num = re.captures(line);
        if num.is_some() {
            let id = num.as_ref().unwrap()[1].trim().parse::<usize>().unwrap();
            let name = num.unwrap()[2].trim().to_string();
            set.insert((id, name));
        }
    }
    for id in set {
        ss_ids.push(id.clone());
        id_list.push((id.0, &JobType::Deploy));
    }
    return ss_ids;
}

fn pretty_print_ids(ids: Vec<(usize, String)>){
    for id in ids {
        println!("{} - {}", id.0, id.1);
    }
}

fn main() {
    //list_files();
    //let job_ID = get_input("Enter the deploy job ID.");
    //println!("{}", job_ID);
    //let path = Path::new(TEST_LOG);
    let path = Path::new(SOFTWARE_MANAGEMENT_LOG);
    let lines = read_log_lines(path);
    //let filtered = search_logs_for_deploy_job(lines, &job_ID.trim()); 
    //println!("{}", filtered.len());
    let mut id_list: Vec<(usize, &JobType)> = Vec::new();
    let self_service_jobs = get_selfservice_install_ids(&lines, &mut id_list);
    let deploy_service_jobs = get_deploy_install_ids(&lines, &mut id_list);
    //let self_service_id = get_input("Which Self Service Install would you like to see lines for?");
    println!("{}", "Self Service Jobs:");
    pretty_print_ids(self_service_jobs);
    println!("{}", "\nDeploy Jobs:");
    pretty_print_ids(deploy_service_jobs);
    let job_id = get_input("Enter the job ID you want to see logs for.");
    search_logs_for_job_id(&lines, &job_id.trim(), &id_list);
}

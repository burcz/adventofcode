use regex::Regex;

use crate::utils::line_reader::read_lines;
use std::collections::HashMap;
use num::integer::lcm;

pub fn a() {
    let mut instructions: Vec<char> = vec![];
    let mut parsed = HashMap::new();
    if let Ok(lines) = read_lines("./input08") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    continue;
                }
                if instructions.len() == 0 {
                    instructions = ip.chars().collect::<Vec<char>>();
                    continue;
                }
                let parsed_line = parse_line(&ip);

                for (k, v) in parsed_line.into_iter() {
                    parsed.insert(k, v);
                }
            }
        }
    }
    let mut current: String = "AAA".to_string();
    let mut step_count = 0;
    while current != "ZZZ".to_string() {
        let instruction = instructions[step_count % instructions.len()];
        current = step(&current, &instruction, &parsed);
        step_count += 1;
    }
    //println!("{}",instruction);
    println!("A: {:?}", step_count);
}

pub fn b() {
    let mut instructions: Vec<char> = vec![];
    let mut points: HashMap<String, Vec<String>> = HashMap::new();
    let mut parsed = HashMap::new();
    if let Ok(lines) = read_lines("./input08") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    continue;
                }
                if instructions.len() == 0 {
                    instructions = ip.chars().collect::<Vec<char>>();
                    continue;
                }
                let parsed_line = parse_line(&ip);

                for (k, v) in parsed_line.into_iter() {
                    parsed.insert(k, v);
                }
            }
        }
    }
    points = get_starting_nodes(&parsed);
    let mut currentz = points.get(&"start".to_string()).unwrap().clone();
    let mut step_count = 0;
    let mut end = points.get(&"end".to_string()).unwrap().clone();
    currentz.sort();
    end.sort();
    let mut step_counts: Vec<usize> = vec![];
    for i in 0..currentz.len() {
        step_count = 0;
        while step_count < 100000 {
            let instruction = instructions[step_count % instructions.len()];
                currentz[i] = step(&currentz[i], &instruction, &parsed);
            step_count += 1;
            if end.contains(&currentz[i]) {
                break;
            }

        }
        step_counts.append(&mut vec![step_count]);
    }
    let mut lcm_step = step_counts.pop().unwrap();
    while step_counts.len() > 0 {
        let s = step_counts.pop().unwrap();
        lcm_step = lcm(lcm_step, s);
    }
    println!("B: {:?}", lcm_step);
}

fn get_starting_nodes(data: &HashMap<String, HashMap<String, String>>) -> HashMap<String, Vec<String>> {
    let mut res: HashMap<String,Vec<String>> = HashMap::new();
    let mut starting: Vec<String> = vec![];
    let mut ending: Vec<String> = vec![];
    let re_starting = Regex::new(r".*A$").unwrap();
    let re_ending = Regex::new(r".*Z$").unwrap();
    for key in data.keys() {
        for cap in re_starting.captures_iter(key) {
            starting.append(&mut vec![cap[0].to_string()]);
        }
        for cap in re_ending.captures_iter(key) {
            ending.append(&mut vec![cap[0].to_string()]);
        }
    }
    res.insert("start".to_string(), starting);
    res.insert("end".to_string(), ending);
    res
}

fn parse_line(line: &String) -> HashMap<String, HashMap<String, String>> {
    let mut res: HashMap<String,HashMap<String,String>> = HashMap::new();
    let mut sides: HashMap<String,String> = HashMap::new();
    sides.insert("L".to_string(), line.split(" = (").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>()[0].to_string());
    sides.insert("R".to_string(), line.split(" = (").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>()[1].strip_suffix(")").unwrap().to_string());
    res.insert(line.split(" = ").collect::<Vec<&str>>()[0].to_string(), sides);
    res
}

fn step(current: &String, instruction: &char, data: &HashMap<String, HashMap<String, String>>) -> String {
    let step = instruction.to_string();
    data.get(current).unwrap().get(&step).unwrap().to_string()
}

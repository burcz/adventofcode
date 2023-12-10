use crate::utils::line_reader::read_lines;
use std::collections::HashMap;

pub fn a() {
    let mut instruction: String = "".to_string();
    let mut res = 0;
    let mut parsed = HashMap::new();
    if let Ok(lines) = read_lines("./input08") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    continue;
                }
                if instruction.len() == 0 {
                    instruction = ip;
                    continue;
                }
                let parsed_line = parse_line(&ip);

                for (k, v) in parsed_line.into_iter() {
                    parsed.insert(k, v);
                }
            }
        }
    }
    let mut current: Vec<String> = vec!["0".to_string(), "AAA".to_string()];
    while current[1] != "ZZZ".to_string() {
        current = step(current[0].parse::<u32>().unwrap(), &current[1], &instruction, &parsed);
    }
    println!("{}",instruction);
    println!("A: {:?}", current[0].parse::<u32>().unwrap());
}

pub fn b() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input08") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
}

fn parse_line(line: &String) -> HashMap<String, HashMap<String, String>> {
    let mut res: HashMap<String,HashMap<String,String>> = HashMap::new();
    let mut sides: HashMap<String,String> = HashMap::new();
    sides.insert("L".to_string(), line.split(" = (").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>()[0].to_string());
    sides.insert("R".to_string(), line.split(" = (").collect::<Vec<&str>>()[1].split(", ").collect::<Vec<&str>>()[1].strip_suffix(")").unwrap().to_string());
    res.insert(line.split(" = ").collect::<Vec<&str>>()[0].to_string(), sides);
    res
}

fn step(mut step_count: u32, current: &String, instruction: &String, data: &HashMap<String, HashMap<String, String>>) -> Vec<String> {
    let mut cur = current.clone();
    for c in instruction.chars() {
        let step = c.to_string();
        cur = data.get(&cur).unwrap().get(&step).unwrap().to_string();
        println!("{}", cur);
        step_count += 1;
        if cur == "ZZZ".to_string() {
            break;
        }
    }
    return vec![step_count.to_string(), cur];
}

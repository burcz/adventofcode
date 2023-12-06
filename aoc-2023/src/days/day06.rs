use crate::utils::line_reader::read_lines;
use std::str::FromStr;

pub fn a() {
    let mut res = 1;
    let mut parsed: Vec<Vec<u128>> = vec![];
    if let Ok(lines) = read_lines("./input06") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_input_a(ip)]);
            }
        }
    }
    for i in 0..parsed[0].len() {
        res *= get_possible_distances(parsed[0][i], parsed[1][i]).len();
    }
    println!("A: {:?}", res);
}

pub fn b() {
    let mut parsed: Vec<u128> = vec![];
    if let Ok(lines) = read_lines("./input06") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_input_b(ip)]);
            }
        }
    }
    let res = get_possible_distances(parsed[0], parsed[1]).len();
    println!("B: {:?}", res);
}

fn parse_input_a(line: String) -> Vec<u128> {
    line.split(":").collect::<Vec<&str>>()[1].to_string().split_whitespace().map(|n| <u128 as FromStr>::from_str(n).unwrap()).collect::<Vec<u128>>()
}

fn parse_input_b(line: String) -> u128 {
    line.split(":").collect::<Vec<&str>>()[1].to_string().split_whitespace().collect::<Vec<&str>>().join("").parse::<u128>().unwrap()
}

fn get_possible_distances(time: u128, record: u128) -> Vec<u128> {
    //println!("time {}, record {}", time, record);
    let mut res: Vec<u128> = vec![];
    for t in 0..=time {
        let d = (time - t) * t;
        //println!("{}",d);
        if d > record {
            res.append(&mut vec![d]);
        }
    }
    res
}

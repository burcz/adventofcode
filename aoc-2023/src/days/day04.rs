use crate::utils::line_reader::read_lines;
use core::str::FromStr;
use std::collections::HashMap;

pub fn a() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input04") {
        for line in lines {
            if let Ok(ip) = line {
                let parsed_line = parse_input_line(&ip);
                let match_count = get_match_count(&parsed_line);
                if match_count > 0 {
                    res += 2u32.pow(match_count - 1);
                }
            }
        }
    }
    println!("A: {:?}", res);
}

pub fn b() {
    let mut ans = 0;
    let mut res = HashMap::new();
    for i in 1..220 {
        res.insert(i, 0);
    }
    let mut line_num = 0;
    if let Ok(lines) = read_lines("./input04") {
        for line in lines {
            if let Ok(ip) = line {
                line_num += 1;
                res.get_mut(&line_num).map(|val| { *val += 1; });
                let parsed_line = parse_input_line(&ip);
                let match_count = get_match_count(&parsed_line);
                //println!("{}|{}|{}", line_num, match_count, res.get(&line_num).unwrap());
                for _ in 0..*res.get(&line_num).unwrap() {
                    for i in 1..=match_count {
                        res.get_mut(&(line_num + i)).map(|val| { *val += 1; });
                        //res[&next] += 1;
                    }
                }

            }
        }
    }
    for v in res.values() {
        ans += v;
    }
    println!("B: {:?}", ans);
}

fn parse_input_line(line: &String) -> Vec<Vec<u32>> {

    let data = line.split(":").collect::<Vec<&str>>()[1];
    let numbers = data.split("|").collect::<Vec<&str>>();
    let theirs = numbers[0].split_whitespace().map(|n| <u32 as FromStr>::from_str(n).unwrap()).collect::<Vec<u32>>();
    let own = numbers[1].split_whitespace().map(|n| <u32 as FromStr>::from_str(n).unwrap()).collect::<Vec<u32>>();

    return vec![theirs, own];
}

fn get_match_count(data: &Vec<Vec<u32>>) -> u32  {
    let mut match_count = 0;
    let theirs = data.get(0).unwrap();
    for n in theirs {

        if data.get(1).unwrap().contains(n) {
            match_count += 1;
        }
    }
    match_count
}

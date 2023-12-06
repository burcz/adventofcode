use std::str::FromStr;
use std::collections::HashMap;

use regex::Regex;

use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut line_vec: Vec<String>= vec![];
    if let Ok(lines) = read_lines("./input05-def") {
        for line in lines {
            if let Ok(ip) = line {
                line_vec.append(&mut vec![ip]);
            }
        }
    }
    let data = parse_input(&line_vec);
    let mut nums: Vec<u128> = data[0][0].to_vec();
    let mut indexes = HashMap::new();
    for i in 1..data.len() {
        gen_new_indexes(&mut indexes, &data[i]);
        //println!("{:?}", indexes);
        //println!("{:?}", nums);
        nums = get_next_values(nums,i, &data);
    }
    println!("A: {:?}", nums.iter().min().unwrap());
}

pub fn b() {
    let mut line_vec: Vec<String>= vec![];
    if let Ok(lines) = read_lines("./input05") {
        for line in lines {
            if let Ok(ip) = line {
                line_vec.append(&mut vec![ip]);
            }
        }
    }
    let data = parse_input(&line_vec);
    let mut ans: u128 = 0;
    //let seeds = get_seeds(&data[0][0].to_vec());
    //println!("{:?}", seeds);
    //let mut locations = get_locations(data[data.len() - 1].to_vec());
    //locations.append(&mut (0..locations.iter().min().unwrap() - 1).collect::<Vec<u128>>());
    //locations.sort();
    //println!("{}",seeds.len());
    //println!("{}",locations.len());
    let mut i = 69000000;
    while ans == 0 {
        if i % 100000 ==0 {
            println!("{}",i);
        }
        let mut calc = i + 1;
        let location = calc;
        for i in (1..data.len()).rev() {
            let vectors: Vec<Vec<u128>> = data[i].to_vec();
            calc = get_prev_value(calc, vectors);
        }
        if seed_exists(calc, &data[0][0]) {
            ans = location;
            break;
        }
        i += 1;
    }

    println!("B: {:?}", ans);
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Vec<u128>>> {
    let mut res: Vec<Vec<Vec<u128>>> = vec![];
    res.append(&mut vec![vec![lines[0].split(": ").nth(1).unwrap().split_whitespace().collect::<Vec<&str>>().iter().map(|n| <u128 as FromStr>::from_str(n).unwrap()).collect::<Vec<u128>>()]]);
    let re = Regex::new(r":$").unwrap();
    let mut index = 0;
    for l in 1..lines.len() {
        if lines[l] == "" {
            continue;
        }
        if re.is_match(&lines[l]) {
            index += 1;
            res.append(&mut vec![vec![]]);
            continue;
        }
        res[index].append(&mut vec![lines[l].split_whitespace().collect::<Vec<&str>>().iter().map(|n| <u128 as FromStr>::from_str(n).unwrap()).collect::<Vec<u128>>()]);

    }
    return res;

}

fn get_next_values(nums: Vec<u128>, check_index: usize, data: &Vec<Vec<Vec<u128>>>) -> Vec<u128> {
    let mut res: Vec<u128> = vec![];
    let mut found = false;
    for s in nums {
        for t_v in &data[check_index] {
            found = false;
            if s >= t_v[1] && s <= t_v[1] + t_v[2] {
                //println!("{:?}", t_v);
                res.append(&mut vec![t_v[0] + s - t_v[1]]);
                found = true;
                break;
            }
        }
        if !found {
            res.append(&mut vec![s]);
        }
    }
    res
}

fn seed_exists(value: u128, data: &Vec<u128>) -> bool {
    for i in (0..data.len()).step_by(2) {
        if value >= data[i] && value <= data[i] + data[i+1] {
            return true;
        }
    }
    false

}


fn get_prev_value(value: u128, prev_vectors: Vec<Vec<u128>>) -> u128 {
    for p_v in prev_vectors {
        //let range = (p_v[0]..=p_v[0]+p_v[2]).collect::<Vec<u128>>();

        //if range.contains(&value) {
        if value >= p_v[0] && value <= p_v[0]+p_v[2] {
            return p_v[1] + value - p_v[0];
        }
    }
    return value;
}


fn gen_new_indexes(indexes: &mut HashMap<u128,u128>, data: &Vec<Vec<u128>>) {
    //println!("{:?}",data);
    for v in data {
    //println!("{:?}",v);
        for d in 0..v[2] {
            if let Some(val) = indexes.remove(&(v[1] + d)) {
                //println!("removing old{}", v[1] + d);
                //println!("inserting old{}", v[0] + d);
                indexes.insert(v[0] + d, val);
            }
            else {
                //println!("inserting new{}", v[0] + d);
                indexes.insert(v[0] + d, v[1] + d);
            }
        }
    }
}


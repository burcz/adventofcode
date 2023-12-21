use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut parsed: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./input15") {
        for line in lines {
            if let Ok(ip) = line {
                parsed = parse_input(ip);
            }
        }
    }
    //println!("{:?}", parsed);
    let mut res = 0;
    for data in parsed {
        res += hash_data(&data);
    }

    println!("A: {:?}", res);
}

pub fn b() {
    let res = 0;
    if let Ok(lines) = read_lines("./input15-def") {
        for line in lines {
            if let Ok(_ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
}

fn parse_input(line: String) -> Vec<String> {
    line.split(",").map(|e| e.to_string()).collect::<Vec<String>>()
}

fn hash_data(data: &String) -> usize {
    let mut hash: usize = 0;
    for c in data.chars() {
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

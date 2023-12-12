use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut parsed: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines("./input11-def") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_lines(&ip)]);
            }
        }
    }
    println!("A: {:?}", parsed);
}

pub fn b() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input11") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
}

fn parse_lines(line: &String) -> Vec<String> {
    line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
}

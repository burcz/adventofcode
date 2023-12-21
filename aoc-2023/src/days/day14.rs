use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut parsed: Vec<Vec<String>> = vec![];

    if let Ok(lines) = read_lines("./input14") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_input(ip)]);
            }
        }
    }
    //print_table(&parsed);
    let mut is_moved = true;
    while is_moved {
        let mut is_moved_lines: Vec<bool> = vec![];
        for to in 0..parsed.len() - 1 {
            is_moved_lines.append(&mut vec![move_line(to, &mut parsed)]);
        }
        is_moved = is_moved_lines.into_iter().reduce(|a, b| a || b).unwrap();
    }

    //print_table(&parsed);
    let load = get_load(&parsed);
    println!("A: {:?}", load);
}

pub fn b() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input14") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
}

fn parse_input(line: String) -> Vec<String> {
    line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
}

fn move_line(to_num: usize, table: &mut Vec<Vec<String>>) -> bool  {
    let mut is_moved = false;
    for i in 0..table[0].len() {
        if &table[to_num + 1][i] == "O" && &table[to_num][i] =="." {
            table[to_num][i] = "O".to_string();
            table[to_num + 1][i] = ".".to_string();
            is_moved = true;
            continue;
        }
    }
    is_moved
}

fn print_table(table: &Vec<Vec<String>>) {
    for l in table {
        println!("{:?}", l.join(""));
    }
    println!("-----------------------");
}

fn get_load(table: &Vec<Vec<String>>) -> usize {
    let mut load: usize = 0;
    let max = table.len();
    for l in 0..table.len() {
        for c in &table[l] {
            if c.clone() == "O".to_string() {
                load += max - l;
            }
        }
    }
    load
}

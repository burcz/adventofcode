use num::ToPrimitive;

use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut parsed: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines("./input11") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_lines(&ip)]);
            }
        }
    }
    //for l in &parsed {
        //println!("{:?}", l.join(""));
    //}
    expand_space(&mut parsed);
    //println!("--------------------------");
    //for l in &parsed {
        //println!("{:?}", l.join(""));
    //}
    let galaxies = find_galaxies(&parsed);
    let dist = calc_distance(&galaxies);
    println!("A: {}", dist);
}

pub fn b() {
    let mut parsed: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines("./input11") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_lines(&ip)]);
            }
        }
    }
    let galaxies = find_galaxies(&parsed);
    let spaces = find_empty_spaces(&parsed);
    //println!("{:?}",galaxies);

    //println!("{:?}", spaces);
    let mut dist: Vec<usize> = vec![0, 0];
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            let res = calc_distance_with_space(&spaces, &vec![galaxies[i].clone(),galaxies[j].clone()]);
            //println!("{:?}", res);
            dist[0] += res[0];
            dist[1] += res[1];
        }
    }
    //println!("{:?}", dist);
    println!("B: {:?}", dist[0] + dist[1] * 999999);
}

fn parse_lines(line: &String) -> Vec<String> {
    line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
}

fn expand_space(data: &mut Vec<Vec<String>>) {
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];
    for c in 0..data[0].len() {
        let mut next_col = false;
        for r in 0..data.len() {
            if data[r][c] != ".".to_string() {
                //println!("found {} {}", c, r);
                next_col = true;
                break;
            }
        }
        if !next_col {
            //println!("adding col {}", c);
            cols.append(&mut vec![c]);
        }
    }
    for c in cols.iter().rev() {
        //println!("inserting to {}", c);
        for r in 0..data.len() {
            data[r].insert(*c, ".".to_string());
        }
    }
    let raw_line: Vec<String> = vec![".".to_string(); data[0].len()];
    for l in 0..data.len() {
        if data[l] == vec![".".to_string(); data[0].len()] {
            rows.append(&mut vec![l]);
        }
    }
    for l in rows.iter().rev() {
        data.insert(*l, raw_line.clone());
    }
}

fn find_galaxies(data: &Vec<Vec<String>>) -> Vec<Vec<isize>> {
    let mut res: Vec<Vec<isize>> = vec![];
    for c in 0..data[0].len() {
        for r in 0..data.len() {
            if data[r][c] != ".".to_string() {
                res.append(&mut vec![vec![c.try_into().unwrap(), r.try_into().unwrap()]]);
            }
        }
    }
    res
}

fn find_empty_spaces(data: &Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];
    for c in 0..data[0].len() {
        let mut next_col = false;
        for r in 0..data.len() {
            if data[r][c] != ".".to_string() {
                //println!("found {} {}", c, r);
                next_col = true;
                break;
            }
        }
        if !next_col {
            //println!("adding col {}", c);
            cols.append(&mut vec![c]);
        }
    }
    for l in 0..data.len() {
        if data[l] == vec![".".to_string(); data[0].len()] {
            rows.append(&mut vec![l]);
        }
    }
    vec![cols,rows]
}

fn calc_distance(galaxies: &Vec<Vec<isize>>) -> isize {
    let mut dist = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            dist += (galaxies[j][0] - galaxies[i][0]).abs() + (galaxies[j][1] - galaxies[i][1]).abs();
        }
    }
    dist
}

fn calc_distance_with_space(spaces: &Vec<Vec<usize>>, galaxies: &Vec<Vec<isize>>) -> Vec<usize> {
    let mut millions: usize = 0;
    let c_min = galaxies[0][0].min(galaxies[1][0]).to_usize().unwrap();
    let c_max = galaxies[0][0].max(galaxies[1][0]).to_usize().unwrap();
    let r_min = galaxies[0][1].min(galaxies[1][1]).to_usize().unwrap();
    let r_max = galaxies[0][1].max(galaxies[1][1]).to_usize().unwrap();
    let d = c_max - c_min + r_max - r_min;
    for c in &spaces[0] {
        if c < &c_min || c > &c_max {
            continue;
        }
        millions += 1;
    }
    for r in &spaces[1] {
        if r < &r_min || r > &r_max {
            continue;
        }
        millions += 1;
    }
    vec![d.to_usize().unwrap(), millions]
}

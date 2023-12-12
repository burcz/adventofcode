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
    let mut dist = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            dist += (galaxies[j][0] - galaxies[i][0]).abs() + (galaxies[j][1] - galaxies[i][1]).abs();
        }
    }
    println!("A: {}", dist);
}

pub fn b() {
    let res = 0;
    if let Ok(lines) = read_lines("./input11") {
        for line in lines {
            if let Ok(_ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
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

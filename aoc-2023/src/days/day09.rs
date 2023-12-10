use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut res: isize = 0;
    let mut parsed: Vec<Vec<Vec<isize>>> = vec![];
    if let Ok(lines) = read_lines("./input09") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![vec![parse_line(&ip)]]);
            }
        }
    }
    for i in 0..parsed.len() {
        let mut deltas = parsed[i][0].clone();
        loop {
            deltas = get_deltas(&deltas);
            parsed[i].append(&mut vec![deltas.clone()]);
            if deltas.iter().sum::<isize>() == 0 && *deltas.last().unwrap() == 0 {
                break;
            }
        }
    }
    for i  in 0..parsed.len() {
        for j in 0..parsed[i].len() {
            res += parsed[i][j].last().unwrap();
        }
    }

    println!("A: {:?}", res);
}

pub fn b() {
    let mut res: isize = 0;
    let mut parsed: Vec<Vec<Vec<isize>>> = vec![];
    if let Ok(lines) = read_lines("./input09") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![vec![parse_line(&ip)]]);
            }
        }
    }
    for i in 0..parsed.len() {
        let mut deltas = parsed[i][0].clone();
        loop {
            deltas = get_deltas(&deltas);
            parsed[i].append(&mut vec![deltas.clone()]);
            if deltas.iter().sum::<isize>() == 0 && *deltas.last().unwrap() == 0 {
                break;
            }
        }
    }
    for j in 0..parsed.len() {
        for i in (1..parsed[j].len()).rev() {
            let first_elem = *parsed[j][i-1].first().unwrap() - *parsed[j][i].first().unwrap();
            let mut revesed = parsed[j][i-1].clone();
            revesed.reverse();
            revesed.append(&mut vec![first_elem]);
            revesed.reverse();
            parsed[j][i-1] = revesed;
        }
    }
    for i in 0..parsed.len() {
        let delta = parsed[i].first().unwrap().first().unwrap();
        //println!("{}", delta);
        res += delta;
    }
    println!("B: {:?}", res);
}

fn parse_line(line: &String) -> Vec<isize> {
    line.split_whitespace().collect::<Vec<&str>>().iter().map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

fn get_deltas(v: &Vec<isize>) -> Vec<isize> {
    let mut res: Vec<isize> = vec![];
    for i in 1..v.len() {
        res.append(&mut vec![v[i]-v[i-1]]);
    }
    res
}

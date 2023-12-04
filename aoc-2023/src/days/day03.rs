use std::{collections::HashMap, str::FromStr};
use regex::Regex;
use crate::utils::line_reader::read_lines;


pub fn a() {
    let mut res = 0;
    let mut parsed: Vec<Vec<char>> = vec![];
    let mut nums: Vec<Vec<HashMap<&str,usize>>> = vec![];
    let mut line_number = 0;
    if let Ok(lines) = read_lines("./input03") {
        for line in lines {
            if let Ok(ip) = line {
                nums.append(&mut vec![get_numbers_from_line(&ip, line_number)]);
                parsed.append(&mut vec![parse_input_line(ip)]);
                line_number += 1;
            }
        }
    }
    let x_max = parsed[0].len() - 1;
    let y_max = parsed.len() - 1;
    for line in nums {
        for num in line {
            if num.is_empty() {
                continue;
            }

            let coordinates = get_neighbor_indexes(*num.get("start").unwrap(), *num.get("end").unwrap(), *num.get("line").unwrap(), x_max, y_max);
            let (inc, _i, _, _) = check_neighbors(&coordinates, *num.get("num").unwrap(), &parsed);
            //if inc == 0 {
            //    println!("{:?}|{:?}", num,coordinates);
            //}
            res += inc;
        }
    }


    println!("A: {:?}", res);
}

pub fn b() {
    let mut stars: Vec<Vec<(usize,usize)>> = vec![vec![(0 as usize, 0 as usize); 140]; 140];
    let mut res = 0;
    let mut parsed: Vec<Vec<char>> = vec![];
    let mut nums: Vec<Vec<HashMap<&str,usize>>> = vec![];
    let mut line_number = 0;
    if let Ok(lines) = read_lines("./input03") {
        for line in lines {
            if let Ok(ip) = line {
                nums.append(&mut vec![get_numbers_from_line(&ip, line_number)]);
                parsed.append(&mut vec![parse_input_line(ip)]);
                line_number += 1;
            }
        }
    }
    let x_max = parsed[0].len() - 1;
    let y_max = parsed.len() - 1;
    for line in nums {
        for num in line {
            if num.is_empty() {
                continue;
            }

            let coordinates = get_neighbor_indexes(*num.get("start").unwrap(), *num.get("end").unwrap(), *num.get("line").unwrap(), x_max, y_max);
            let (_, sx, sy, star) = check_neighbors(&coordinates, *num.get("num").unwrap(), &parsed);
            //if star > 0 {
            //    stars[sy][sx] += 1;
            //}
            if stars[sy][sx].1 == 0 {
                stars[sy][sx] = (1, star);
            }
            else {
                //println!("{}:{}",sx,sy);
                stars[sy][sx] = (stars[sy][sx].0 + 1, stars[sy][sx].1 * star);
            }
            //if inc == 0 {
            //    println!("{:?}|{:?}", num,coordinates);
            //}
        }
    }
    for l in stars {
        //for k in l {
        //    if k == 1 {
        //        println!("dfgs");
        //    }
        //}
        for k in l {
            if k.0 == 2 {
                res += k.1;
            }
        }
        //res += l.iter().copied().reduce(|a.1, b.1| a + b).unwrap();
    }

    println!("B: {}", res);
}

fn parse_input_line(line: String) -> Vec<char> {
    return line.chars().collect::<Vec<_>>();
}

fn get_numbers_from_line<'a>(line: &String, line_number: usize) -> Vec<HashMap<&'a str,usize>> {
    let mut res = vec![];
    let re = Regex::new(r"(?<num>\d+)").unwrap();
    for cap in re.captures_iter(&line) {
        let mut num = HashMap::new();
        num.insert("num", FromStr::from_str(&cap["num"]).unwrap());
        num.insert("start", cap.get(0).unwrap().start());
        num.insert("end", cap.get(0).unwrap().end() - 1);
        num.insert("line", line_number);
        res.append(&mut vec![num]);
    }
    res
}

fn check_neighbors(coordinates: &Vec<(usize,usize)>, num: usize, table: &Vec<Vec<char>>) -> (usize, usize, usize, usize) {
    let mut val;
    for c in coordinates {
        val = table[c.1][c.0];
        if val.to_string() != "." {
            if val.to_string() == "*" {
                return (num, c.1, c.0, num);
            }
            return (num, 0, 0, 0);
        }
    }
    return (0, 0, 0, 0);
}

fn get_neighbor_indexes(x1: usize, x2: usize, y1: usize, x_max: usize, y_max: usize) -> Vec<(usize,usize)> {
    let mut coords: Vec<(usize,usize)> = vec![];
    let x_range = x1 as isize..=x2 as isize;
    for x in x1 as isize -1..=x2 as isize+1 {
        for y in y1 as isize-1..=y1 as isize + 1 {
            if x_range.contains(&x) && y == y1 as isize {
                continue;
            }
            if x >=0 && x <= x_max as isize && y >= 0 && y<= y_max as isize {
                coords.append(&mut vec![(x as usize,y as usize)]);
            }
        }
    }
    coords
}

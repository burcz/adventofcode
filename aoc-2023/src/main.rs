use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        println!("{}", a(lines));
    }
    if let Ok(lines) = read_lines("./input") {
        println!("{}", b(lines));
    }
}

fn a(lines: Lines<io::BufReader<File>>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut first: u32 = 10;
        let mut last: u32 = 0;
        if let Ok(ip) = line {
            first = 10;
            last = 0;
            for (_, c) in ip.chars().enumerate() {
                if c.is_numeric() {
                    if first == 10 {
                        first = c.to_digit(10).unwrap();
                    }
                    last = c.to_digit(10).unwrap();
                }
            }
        }
        sum += first * 10 + last;
    }
    sum
}

fn b(lines: Lines<io::BufReader<File>>) -> u32 {
    let translator: HashMap<&str,u32> = [
                                   ("one", 1),
                                   ("two", 2),
                                   ("three", 3),
                                   ("four", 4),
                                   ("five", 5),
                                   ("six", 6),
                                   ("seven", 7),
                                   ("eight", 8),
                                   ("nine", 9),
    ].into();
    let rel = Regex::new(r"(?<digit>one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let rer = Regex::new(r"(?<digit>eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let mut sum = 0;
    for line in lines {
        let mut found_first = false;
        let mut found_last = false;
        let mut first = HashMap::new();
        let mut last = HashMap::new();

        let mut found_first_str = false;
        let mut found_last_str = false;
        let mut first_str = HashMap::new();
        let mut last_str = HashMap::new();
        let mut num: u32 = 0;
        if let Ok(ip) = line {
            println!("{}",ip);
            match rel.captures(&ip) {
                Some(capl) => {
                    let start = capl.get(0).unwrap().start() as u32;
                    let digit = &capl["digit"];
                    let translated = translator.get(digit).unwrap();

                    first_str.insert(start, *translated);
                }
                None => {
                    // The regex did not match. Deal with it here!
                }
            }


            let rev_ip = ip.chars().rev().collect::<String>();
            println!("{}",rev_ip);
            match rer.captures(&rev_ip) {
                Some(capr) => {
                    let start = capr.get(0).unwrap().start() as u32;

                    let non_rev = capr["digit"].chars().rev().collect::<String>();
                    println!("{}",non_rev);
                    let digit = translator.get(&non_rev.as_str()).unwrap();
                    last_str.insert(u32::try_from(ip.len()).unwrap() - start, *digit);
                }
                None => {
                    // The regex did not match. Deal with it here!
                }
            }



            for (i, c) in ip.chars().enumerate() {
                if c.is_numeric() {
                    if !found_first{
                        first.insert(i as u32, u32::try_from(c.to_digit(10).unwrap()).unwrap());
                        found_first = true;
                    }
                        last.insert(i as u32, c.to_digit(10).unwrap());
                }
            }
            first.extend(first_str);
            last.extend(last_str);
            let mut sorted_i: Vec<_> = first.iter().collect();
            sorted_i.sort_by_key(|a| a.0);
            let (i, _) = sorted_i[0];
            let mut sorted_j: Vec<_> = last.iter().collect();
            sorted_j.sort_by_key(|a| a.0);
            sorted_j.reverse();
            let (j, _) = sorted_j[0];
            num = first.get(i).unwrap() * 10 + last.get(j).unwrap();
            println!("{}", num);
        }
        sum += num;
    }
    return sum;
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

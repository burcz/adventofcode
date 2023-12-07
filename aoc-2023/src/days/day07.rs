use phf::phf_map;
use std::collections::VecDeque;

use crate::utils::line_reader::read_lines;


static CARDS: phf::Map<&'static str, u32> = phf_map! {
     "2"=> 2,
     "3"=> 3,
     "4"=> 4,
     "5"=> 5,
     "6"=> 6,
     "7"=> 7,
     "8"=> 8,
     "9"=> 9,
     "T"=> 10,
     "J"=> 11,
     "Q"=> 12,
     "K"=> 13,
     "A"=> 14
};

pub fn a() {
    let mut res = 0;
    let mut parsed: Vec<Vec<Vec<u32>>> = vec![];
    if let Ok(lines) = read_lines("./input07") {
        for line in lines {
            if let Ok(ip) = line {
                let hand = parse_line(&ip);
                //println!("{:?}",hand);
                let parsed_hand = check_hand(&hand[0]);
                let hand_type = get_hand_type(&parsed_hand);
                //println!("hand: {:?}", parsed_hand);
                //println!("type: {:?}", hand_type);
                parsed.append(&mut vec![vec![hand[0].clone(), hand[1].clone(), Vec::from([hand_type])]]);
            }
        }
    }
    parsed.sort_unstable_by(|a, b| {
        if b[2][0] == a[2][0] {
        for i in 0..a[0].len() {
            if b[0][i] == a[0][i] {
                continue;
            }
            return b[0][i].cmp(&a[0][i]);
        }
        }
        b[2][0].cmp(&a[2][0])
    });
    let mut j = 0;
    for i in (0..parsed.len()).rev() {
        //println!("{}", parsed[i][1][0]);
        j += 1;
        res += usize::try_from(parsed[i][1][0]).unwrap() * j;
    }
    println!("A: {:?}", res);
}

pub fn b() {
    let mut parsed: Vec<u128> = vec![];
    if let Ok(lines) = read_lines("./input07-def") {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("B: {:?}", parsed);
}

fn parse_line(line: &String) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = vec![];
    let data = line.split_whitespace().collect::<Vec<&str>>();
    //println!("{:?}", data);
    let mut hand: VecDeque<&str> = data[0].split("").collect::<VecDeque<&str>>();

    hand.pop_back();
    hand.pop_front();
    let parsed_hand = Vec::from(hand.iter().map(|s| CARDS.get(s).cloned().unwrap()).collect::<Vec<u32>>());
    res.append(&mut vec![parsed_hand]);
    res.append(&mut vec![vec![data[1].parse::<u32>().unwrap()]]);
    res
}

fn check_hand(hand: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = vec![];
    let mut h = hand.clone();
    h.sort();
    //println!("{:?}",h);
    let mut pairs: Vec<u32> = vec![];
    let mut tri: Vec<u32> = vec![];
    let mut quar: Vec<u32> = vec![];
    let mut quin: Vec<u32> = vec![];
    let prev_d = 0;
    let mut d = 0;
    for i in 1..h.len() {
        d = h[i] - h[i - 1];
        if d == 0 {
            if pairs.contains(&h[i]) {
                pairs.pop();
                tri.append(&mut vec![h[i]]);
            }
            else if tri.contains(&h[i]) {
                tri.pop();
                quar.append(&mut vec![h[i]]);
            }
            else if quar.contains(&h[i]) {
                quar.pop();
                quin.append(&mut vec![h[i]]);
            }
            else {
                pairs.append(&mut vec![h[i]]);
            }
        }
    }
    res.append(&mut vec![pairs]);
    res.append(&mut vec![tri]);
    res.append(&mut vec![quar]);
    res.append(&mut vec![quin]);
    res
}

fn get_hand_type(parsed_hand: &Vec<Vec<u32>>) -> u32 {
    if parsed_hand[3].len() > 0 {
        return 6; // five of a kind
    }
    if parsed_hand[2].len() > 0 {
        return 5; // four of a kind
    }
    if parsed_hand[1].len() > 0 {
        if parsed_hand[0].len() > 0 {
            return 4; // full house
        }
        else {
            return 3; // three of a kind
        }
    }
    if parsed_hand[0].len() > 1 {
        return 2; // two pairs
    }
    if parsed_hand[0].len()> 0 {
        return 1; // pair
    }
    return 0; // high card
}


use crate::utils::line_reader::read_lines;
use std::collections::HashMap;

pub fn a() {
    let mut res = 0;
    let mut cubes = HashMap::new();
    cubes.insert("red", 12);
    cubes.insert("green", 13);
    cubes.insert("blue", 14);

    if let Ok(lines) = read_lines("./input02") {
        for line in lines {
            if let Ok(ip) = line {
                let game_num = ip.split(":").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
                let games = parse_games(ip.split(":").collect::<Vec<&str>>()[1]);
                res += game_num;
                for game in games {
                    if game.contains_key("red") && game["red"] > cubes["red"] {
                        res -= game_num;
                        break;
                    }
                    if game.contains_key("green") && game["green"] > cubes["green"] {
                        res -= game_num;
                        break;
                    }
                    if game.contains_key("blue") && game["blue"] > cubes["blue"] {
                        res -= game_num;
                        break;
                    }
                }
            }
        }
    }
    println!("A: {}",res);
}

pub fn b() {
    let mut res = 0;
    let mut cubes = HashMap::new();
    cubes.insert("red", 0);
    cubes.insert("green", 0);
    cubes.insert("blue", 0);

    if let Ok(lines) = read_lines("./input02") {
        for line in lines {
            if let Ok(ip) = line {
                if let Some(x) = cubes.get_mut("red") {
                    *x = 0;
                }
                if let Some(x) = cubes.get_mut("green") {
                    *x = 0;
                }
                if let Some(x) = cubes.get_mut("blue") {
                    *x = 0;
                }
                let game_num = ip.split(":").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
                let games = parse_games(ip.split(":").collect::<Vec<&str>>()[1]);
                for game in games {
                    if game.contains_key("red") && game["red"] > cubes["red"] {
                        if let Some(x) = cubes.get_mut("red") {
                            *x = game["red"];
                        }
                    }
                    if game.contains_key("green") && game["green"] > cubes["green"] {
                        if let Some(x) = cubes.get_mut("green") {
                            *x = game["green"];
                        }
                    }
                    if game.contains_key("blue") && game["blue"] > cubes["blue"] {
                        if let Some(x) = cubes.get_mut("blue") {
                            *x = game["blue"];
                        }
                    }
                }
                res += cubes.get("red").unwrap() * cubes.get("green").unwrap() * cubes.get("blue").unwrap();
            }
        }
    }
    println!("B: {}",res);
}

fn parse_games(game_data: &str) -> Vec<HashMap<&str,u32>> {
    let mut res: Vec<HashMap<&str,u32>> = vec![];
    let games = game_data.split(";").map(|g| g.split_whitespace().rev().collect::<Vec<&str>>()).collect::<Vec<_>>();
    for mut game in games {
        let mut g = HashMap::new();
        for p in (0..game.len()).step_by(2) {
            if game[p].chars().last().unwrap().to_string() == "," {
                game[p] = &game[p][0..game[p].to_string().len() -1];
            }
            g.insert(game[p], game[p + 1].parse::<u32>().unwrap());
        }
        res.append(&mut vec![g]);
    }
    res
}

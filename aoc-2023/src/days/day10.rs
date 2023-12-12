use crate::utils::line_reader::read_lines;

// 0 north
// 1 west
// 2 south
// 3 east

pub fn a() {
    let mut parsed: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines("./input10-def") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_lines(&ip)]);
            }
        }
    }
    let start = get_starting_node(&parsed);
    let first_steps = get_first_steps(&start, &parsed);
    let mut bef_a = start.clone();
    let mut act_a = first_steps[0].clone();
    let mut bef_b = start.clone();
    let mut act_b = first_steps[2].clone();
    let mut step = 1;
    loop {
        //println!("{:?}{:?}", act_a, act_b);
        let next_a = get_next_coord(&bef_a, &act_a, &parsed);
        let next_b = get_next_coord(&bef_b, &act_b, &parsed);
        step += 1;
        if next_a[0] == next_b[0] {
            break;
        }
        bef_a = act_a.clone();
        act_a = next_a[0].clone();
        bef_b = act_b.clone();
        act_b = next_b[0].clone();
    }
    println!("A: {:?}", step);
}

pub fn b() {
    let mut parsed: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines("./input10") {
        for line in lines {
            if let Ok(ip) = line {
                parsed.append(&mut vec![parse_lines(&ip)]);
            }
        }
    }
    let start = get_starting_node(&parsed);
    let first_steps = get_first_steps(&start, &parsed);
    let mut bef = [start.clone(), [5].to_vec()].to_vec();
    let mut act = first_steps.clone();
    let mut step = 1;
    //println!("start: {:?}", start);
    let mut turns: Vec<usize> = [0, 0].to_vec();
    let mut path = parsed.clone();
    path[start[1]][start[0]] = "*".to_string();
    loop {
        //println!("{:?}", act);
        let act_symbol = parsed[act[0][1]][act[0][0]].as_str();
        path[act[0][1]][act[0][0]] = "*".to_string();
        match  act[1][0] {
            0 => {
                match act_symbol {
                    "F" => turns[1] += 1,
                    "7" => turns[0] += 1,
                    _ => ()//println!("no north turn"),
                }
            },
            1 => {
                match act_symbol {
                    "L" => turns[1] += 1,
                    "F" => turns[0] += 1,
                    _ => ()//println!("no west turn"),
                }
            },
            2 => {
                match act_symbol {
                    "J" => turns[1] += 1,
                    "L" => turns[0] += 1,
                    _ => ()//println!("no south turn"),
                }
            },
            3 => {
                match act_symbol {
                    "7" => turns[1] += 1,
                    "J" => turns[0] += 1,
                    _ => ()//println!("no east turn"),
                }
            }
            _ => ()//println!("no turn"),
        }
        //println!("turns: {:?}", turns);

        let next = get_next_coord(&bef[0], &act[0], &parsed);
        step += 1;
        if next[0] == start {
            break;
        }
        bef = act.clone();
        act = next.clone();
    }
    let mut side = "R";
    if turns[0] > turns[1] {
        side = "L";
    }
    bef = [start.clone(), [5].to_vec()].to_vec();
    act = first_steps.clone();
    step = 1;
    loop {
        set_inner(&side, act[1][0], &act[0], &mut parsed, &path);

        let next = get_next_coord(&bef[0], &act[0], &parsed);
        step += 1;
        if next[0] == start {
            break;
        }
        bef = act.clone();
        act = next.clone();

    }


    let mut overwritten = false;
    loop {
        //break;
        let mut i_coords: Vec<Vec<usize>> = vec![];
        for y in 0..parsed.len() {
            for x in 0..parsed[y].len() {
                if parsed[y][x] == "I".to_string() {
                    i_coords.append(&mut vec![[x, y].to_vec()]);
                }
            }
        }
        for c in i_coords {
            //println!("{:?}",c);
            if path[c[1] - 1][c[0]] != "*" && parsed[c[1] - 1][c[0]] != "I" {
                parsed[c[1] - 1][c[0]] = "I".to_string();
                overwritten = true;
            }
            if path[c[1]][c[0] + 1] != "*" && parsed[c[1]][c[0] + 1] != "I" {
                parsed[c[1]][c[0] + 1] = "I".to_string();
                overwritten = true;
            }
            if path[c[1]][c[0] - 1] != "*" && parsed[c[1]][c[0] - 1] != "I" {
                parsed[c[1]][c[0] - 1] = "I".to_string();
                overwritten = true;
            }
            if path[c[1] + 1][c[0]] != "*" && parsed[c[1] + 1][c[0]] != "I" {
                parsed[c[1] + 1][c[0]] = "I".to_string();
                overwritten = true;
            }
        }
        if !overwritten {
            break;
        }
        overwritten = false;
    }
    let mut inner = 0;
    for l in parsed.clone() {
        // println!("{}", l.join(""));
        for c in l {
            if c == "I".to_string() {
                inner += 1;
            }
        }
    }
    for l in path.clone() {
        //println!("{}", l.join(""));
    }
    println!("B: {:?}", inner);
}

fn set_inner(side: &str, direction: usize, c: &Vec<usize>, data: &mut Vec<Vec<String>>, path: &Vec<Vec<String>>) {
    let mut to_set: Vec<Vec<usize>> = vec![];
    let char = data[c[1]][c[0]].as_str();
    match char {
        "|" => {
            if direction == 2 {
                if side == "L" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                }
                else {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                }

            }
            else {
                if side == "R" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                }
                else {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                }

            }
        },
        "-" => {
            if direction == 1 {
                if side == "L" {
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
                else {
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }

            }
            else {
                if side == "R" {
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
                else {
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }

            }
        },
        "L" => {
            if direction == 1 {
                if side == "L" {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
            }
            if direction == 2 {
                if side == "R" {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
            }
        },
        "F" => {
            if direction == 0 {
                if side == "L" {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }
            }
            if direction == 1 {
                if side == "R" {
                    if c[0] > 0 {
                        to_set.append(&mut vec![[c[0] - 1, c[1]].to_vec()]);
                    }
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }
            }
        },
        "J" => {
            if direction == 2 {
                if side == "L" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
            }
            if direction == 3 {
                if side == "R" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                    to_set.append(&mut vec![[c[0], c[1] + 1].to_vec()]);
                }
            }
        },
        "7" => {
            //println!("{:?}{}", c, direction);
            if direction == 3 {
                if side == "L" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }
            }
            if direction == 0 {
                if side == "R" {
                    to_set.append(&mut vec![[c[0] + 1, c[1]].to_vec()]);
                    if c[1] > 0 {
                        to_set.append(&mut vec![[c[0], c[1] - 1].to_vec()]);
                    }
                }
            }
        },
        _ => return,

    }
    //println!("{:?}",to_set);
    for coord in to_set {
        if coord[1] < data.len() && coord[0] < data[0].len() {
            if path[coord[1]][coord[0]] != "*" {
                data[coord[1]][coord[0]] = "I".to_string();
            }
        }
    }
}

fn parse_lines(line: &String) -> Vec<String> {
    line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
}

fn get_starting_node(data: &Vec<Vec<String>>) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == "S".to_string() {
                res = vec![x,y];
            }
        }
    }
    res
}

fn get_first_steps(start: &Vec<usize>, data: &Vec<Vec<String>>) -> Vec<Vec<usize>>{
    let mut res: Vec<Vec<usize>> = vec![];
    if start[1] > 0 {
        let north = data[start[1] - 1][start[0]].as_str();
        match north {
            "|" => res.append(&mut vec![[start[0], start[1] - 1].to_vec(),vec![0]]),
            "F" => res.append(&mut vec![[start[0], start[1] - 1].to_vec(),vec![0]]),
            "7" => res.append(&mut vec![[start[0], start[1] - 1].to_vec(),vec![0]]),
            _ => ()//println!("not north"),
        }
    }
    if start[0] > 0 {
        let west = data[start[1]][start[0] - 1].as_str();
        match west {
            "-" => res.append(&mut vec![[start[0] - 1, start[1]].to_vec(),vec![1]]),
            "F" => res.append(&mut vec![[start[0] - 1, start[1]].to_vec(),vec![1]]),
            "L" => res.append(&mut vec![[start[0] - 1, start[1]].to_vec(),vec![1]]),
            _ => ()//println!("not west"),
        }
    }
    if start[1] < data.len() {
        let south = data[start[1] + 1][start[0]].as_str();
        match south {
            "|" => res.append(&mut vec![[start[0], start[1] + 1].to_vec(),vec![2]]),
            "J" => res.append(&mut vec![[start[0], start[1] + 1].to_vec(),vec![2]]),
            "L" => res.append(&mut vec![[start[0], start[1] + 1].to_vec(),vec![2]]),
            _ => ()//println!("not south"),
        }
    }
    if start[0] < data[0].len() {
        let east = data[start[1]][start[0] + 1].as_str();
        match east {
            "-" => res.append(&mut vec![[start[0] + 1, start[1]].to_vec(),vec![3]]),
            "J" => res.append(&mut vec![[start[0] + 1, start[1]].to_vec(),vec![3]]),
            "7" => res.append(&mut vec![[start[0] + 1, start[1]].to_vec(),vec![3]]),
            _ => ()//println!("not east"),
        }
    }
    res
}

fn get_next_coord(bef: &Vec<usize>, cur: &Vec<usize>, data: &Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = [cur.clone(), vec![5]].to_vec();
    let c = data[cur[1]][cur[0]].as_str();
    //print!("{}",c);
    match c {
        "|" => {
            res[0][1] = 2 * cur[1] - bef[1];
            if bef[1] < cur[1] {
                res[1][0] = 2;
            }
            else {
                res[1][0] = 0;
            }
        },
        "-" => {
            res[0][0] = 2 * cur[0] - bef[0];
            if bef[0] < cur[0] {
                res[1][0] = 3;
            }
            else {
                res[1][0] = 1;
            }
        },
        "L" => {
            if bef[0] == cur[0] {
                res[0][0] += 1;
                res[1][0] = 3;
            }
            if cur[1] == bef[1]  {
                res[0][1] -= 1;
                res[1][0] = 0;
            }
        },
        "J" => {
            if bef[0] == cur[0] {
                res[0][0] -= 1;
                res[1][0] = 1;
            }
            if cur[1] == bef[1] {
                res[0][1] -= 1;
                res[1][0] = 0;
            }
        },
        "F" => {
            if bef[0] == cur[0] {
                res[0][0] += 1;
                res[1][0] = 3;
            }
            if cur[1] == bef[1] {
                res[0][1] += 1;
                res[1][0] = 2;
            }
        },
        "7" => {
            if cur[0] == bef[0] {
                res[0][0] -= 1;
                res[1][0] = 1;
            }
            if cur[1] == bef[1] {
                res[0][1] += 1;
                res[1][0] = 2;
            }
        },
        _ => ()//println!("no match"),
    }
    res
}

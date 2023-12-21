use crate::utils::line_reader::read_lines;

pub fn a() {
    let mut parsed: Vec<Vec<Vec<i8>>> = vec![];
    let mut buffer: Vec<Vec<i8>> = vec![];
    let mut res = 0;
    if let Ok(lines) = read_lines("./input13") {
        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    // println!("|{}|", ip);
                    buffer.append(&mut vec![parse_input(ip)]);
                }
                else {
                    parsed.append(&mut vec![buffer.clone()]);
                    buffer = vec![];
                }
            }
        }
    }
    parsed.append(&mut vec![buffer.clone()]);
    for table in parsed {
        let mut mirr_col = 0;
        let mut mirr_row = 0;
        for c in 1..table[0].len() {
            let foldable = try_fold_at_col(c, &table);
            if foldable {
                mirr_col = c;
                break;
            }
        }
        if mirr_col == 0 {
            for r in 1..table.len() {
                let foldable = try_fold_at_row(r, &table);
                if foldable {
                    mirr_row = r;
                    break;
                }
            }

        }
        //println!("{}", mirr_col);
        //println!("{}", mirr_row);
        res += mirr_row * 100 + mirr_col;
    }
    println!("A: {:?}", res);
}

pub fn b() {
    let res = 0;
    if let Ok(lines) = read_lines("./input13") {
        for line in lines {
            if let Ok(_ip) = line {
            }
        }
    }
    println!("B: {:?}", res);
}

fn parse_input(line: String) -> Vec<i8> {
    line.chars().into_iter().map(|c| {
        if c == '.' {
            return 0;
        }
        1
    }).collect::<Vec<i8>>()
}

fn try_fold_at_col(col: usize, table: &Vec<Vec<i8>>) -> bool {
    let col_count = col.min(table[0].len() - col);
    for row in table {
        let left = &row[col - col_count..col];
        let right = &row[col..col + col_count];
        //println!("L:{:?}|R:{:?}", left, right);
        for i in 0..col_count {
        if left[left.len() - 1 - i] - right[i] != 0 {
            return false;
        }
        }
    }
    true
}

fn try_fold_at_row(row: usize, table: &Vec<Vec<i8>>) -> bool {
    let row_count = row.min(table.len() - row);
    for col in 0..table[0].len() {
        let mut buffer: Vec<i8> = vec![];
        for row in 0..table.len() {
            buffer.append(&mut vec![table[row][col]]);
        }
        let left = &buffer[row - row_count..row];
        let right = &buffer[row..row + row_count];
        //println!("L:{:?}|R:{:?}", left, right);
        for i in 0..row_count {
        if left[left.len() - 1 - i] - right[i] != 0 {
            return false;
        }
        }
    }
    true
}

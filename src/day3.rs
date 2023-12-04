// Should return 539433, currently doesn't
pub fn pt1(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut valid_posns: Vec<Vec<bool>> = Vec::new();
    for _ in 0..lines.len() {
        valid_posns.push(Vec::new());
        for _ in 0..lines[0].len() {
            valid_posns.last_mut().unwrap().push(false);
        }
    }
    for (yidx, line) in lines.iter().enumerate() {
        for (xidx, char) in line.chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                for dx in -1..2 {
                    for dy in -1..2 {
                        if (yidx as i32 + dy) < 0 || (xidx as i32 + dx) < 0 {
                            continue;
                        }
                        let row_opt = valid_posns.get_mut((yidx as i32 + dy) as usize);
                        let row;
                        if row_opt.is_none() {
                            continue;
                        } else {
                            row = row_opt.unwrap();
                        }
                        if row.get((xidx as i32 + dx) as usize).is_some() {
                            row[(xidx as i32 + dx) as usize] = true;
                        }
                    }
                }
            }
        }
    }

    let mut part_nums: Vec<i32> = Vec::new();
    for (yidx, line) in lines.iter().enumerate() {
        let mut start_idx = -1;
        let v_row = valid_posns.get(yidx).unwrap();
        let c_row: Vec<char> = line.chars().collect();
        for (xidx, char) in c_row.iter().enumerate() {
            if start_idx == -1 {
                if char.is_digit(10) {
                    start_idx = xidx as i32;
                }
            } else {
                if !char.is_digit(10) {
                    // Number is encompassed by start_idx..xidx
                    let mut part_num: String = "".to_owned();
                    let mut valid = false;
                    for idx in start_idx as usize..xidx {
                        if v_row[idx] {
                            valid = true;
                        }
                        part_num.push_str(&c_row[idx].to_string());
                    }

                    if valid {
                        part_nums.push(part_num.parse::<i32>().unwrap());
                    }
                    start_idx = -1;
                }
            }
        }
    }

    println!("Sum of parts: {}", part_nums.iter().fold(0, |a,b|a+b));
}

pub fn pt2(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    for (yidx, line) in lines.iter().enumerate() {
        for (xidx, char) in line.chars().enumerate() {
            if char == '*' {
                for dy in -1..2 {
                    if (dy + yidx as i32) < 0 {
                        continue;
                    }
                    let line_opt = lines.get((yidx as i32 + dy) as usize);
                    if let Some(test_line) = line_opt {
                        let test_chars: Vec<char> = test_line.chars().collect();
                        for dx in -1..2 {
                            if (dx + xidx as i32) < 0 {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
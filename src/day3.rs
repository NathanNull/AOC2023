// Should return 539433, currently doesn't
pub fn pt1(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut valid_posns: Vec<Vec<i32>> = Vec::new();
    for _ in 0..lines.len() {
        valid_posns.push(Vec::new());
        for _ in 0..lines[0].len() {
            valid_posns.last_mut().unwrap().push(0);
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
                            row[(xidx as i32 + dx) as usize] += 1;
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
        let mut c_row: Vec<char> = line.chars().collect();
        c_row.push('.'); // So it'll end any nums on the end of the line
        for (xidx, char) in c_row.iter().enumerate() {
            if start_idx == -1 {
                if char.is_digit(10) {
                    start_idx = xidx as i32;
                }
            } else {
                if !char.is_digit(10) {
                    // Number is encompassed by start_idx..xidx
                    let mut part_num: String = "".to_owned();
                    let mut valid = 0;
                    for idx in (start_idx as usize)..xidx {
                        valid += v_row[idx];
                        part_num.push_str(&c_row[idx].to_string());
                    }

                    if valid > 0 {
                        part_nums.push(part_num.parse::<i32>().unwrap());
                    }
                    start_idx = -1;
                    print!("{},", part_num);
                }
            }
        }
        println!();
    }

    println!("Sum of parts: {}", part_nums.iter().fold(0, |a, b| a + b));
}

// Find *s with 2 nums adjacent, multiply nums, return sum of all
// Also too low, but here I don't know what the right answer's supposed to be
pub fn pt2(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut posns: Vec<Vec<Vec<i32>>> = Vec::new();
    for _ in 0..lines.len() {
        posns.push(Vec::new());
        for _ in 0..lines[0].len() {
            posns.last_mut().unwrap().push(Vec::new());
        }
    }
    let mut gear_id = 0;
    for (yidx, line) in lines.iter().enumerate() {
        for (xidx, char) in line.chars().enumerate() {
            if char == '*' {
                for dy in -1..2 {
                    if (dy + yidx as i32) < 0 {
                        continue;
                    }
                    for dx in -1..2 {
                        if (dx + xidx as i32) < 0 {
                            continue;
                        }
                        posns[(dy + yidx as i32) as usize][(dx + xidx as i32) as usize]
                            .push(gear_id);
                    }
                }
                gear_id += 1;
            }
        }
    }

    let mut ratios: Vec<Vec<i32>> = Vec::new();
    for _ in 0..gear_id {
        ratios.push(Vec::new());
    }
    for (yidx, line) in lines.iter().enumerate() {
        let mut start_idx = -1;
        let v_row = &posns[yidx];
        let mut c_row: Vec<char> = line.chars().collect();
        c_row.push('.');
        for (xidx, char) in c_row.iter().enumerate() {
            if start_idx == -1 {
                if char.is_digit(10) {
                    start_idx = xidx as i32;
                }
            } else {
                if !char.is_digit(10) {
                    let mut gears: Vec<i32> = Vec::new();
                    // Number is encompassed by start_idx..xidx
                    let mut part_num: String = "".to_owned();
                    for idx in start_idx as usize..xidx {
                        gears.extend(&v_row[idx]);
                        part_num.push_str(&c_row[idx].to_string());
                    }

                    gears.sort();
                    gears.dedup();

                    if gears.len() > 1 {
                        println!("huh");
                    }

                    for gear in gears {
                        ratios[gear as usize].push(part_num.parse::<i32>().unwrap());
                    }
                    start_idx = -1;
                }
            }
        }
    }

    let calc_ratios: Vec<i32> = ratios
        .iter()
        .filter(|nums| nums.len() > 1)
        .map(|nums| nums.iter().fold(1, |acc, num| acc * num))
        .collect();

    println!(
        "The sum of the gear ratios is {}",
        calc_ratios.iter().fold(0, |acc, num| acc + num)
    );
}

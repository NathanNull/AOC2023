use std::cmp::max;

pub fn pt1(input: String) {
    let lines = input.split("\r\n");
    let mut score = 0;
    for (idx, line) in lines.enumerate() {
        let pulls = line
            .split(": ")
            .nth(1)
            .expect("No colon in a line!")
            .split("; ");
        let mut is_valid = true;
        for pull in pulls {
            let cols = pull.split(", ");
            for col in cols {
                let data: Vec<&str> = col.split(" ").collect();
                let (num, color) = (
                    data[0].parse::<i32>().expect("Non-number pull count"),
                    data[1],
                );
                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Invalid color '{color}'"),
                };
                if num > max {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }
        if is_valid {
            score += idx + 1;
        }
    }
    println!("Score: {score}");
}

pub fn pt2(input: String) {
    let lines = input.split("\r\n");
    let mut score = 0;
    for line in lines {
        let pulls = line
            .split(": ")
            .nth(1)
            .expect("No colon in a line!")
            .split("; ");
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for pull in pulls {
            let cols = pull.split(", ");
            for col in cols {
                let data: Vec<&str> = col.split(" ").collect();
                let (num, color) = (
                    data[0].parse::<i32>().expect("Non-number pull count"),
                    data[1],
                );
                match color {
                    "red" => max_r = max(max_r, num),
                    "green" => max_g = max(max_g, num),
                    "blue" => max_b = max(max_b, num),
                    _ => panic!("Invalid color '{color}'"),
                };
            }
        }
        score += max_r * max_g * max_b;
    }
    println!("Score: {score}");
}

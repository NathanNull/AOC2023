pub fn pt1(input: String) {
    let [time_str, dist_str] = input.split("\r\n").collect::<Vec<&str>>()[..] else {
        panic!("Invalid input")
    };
    let times = time_str.split(" ").filter(|s| !s.is_empty()).skip(1);
    let mut distances = dist_str.split(" ").filter(|s| !s.is_empty()).skip(1);
    let races = times.map(|t| {
        (
            t.parse::<i64>().unwrap(),
            distances.next().unwrap().parse::<i64>().unwrap(),
        )
    });
    let mut product: Vec<i64> = Vec::new();
    for race in races {
        println!("Race: {:?}", race);
        let mut first_worked = -1;
        for pressed in 0..(race.0 + 1) {
            let score = (race.0 - pressed) * pressed;
            if first_worked == -1 {
                if score > race.1 {
                    first_worked = pressed;
                }
            } else if score <= race.1 {
                println!("Millis {} to {} worked", first_worked, pressed);
                product.push(pressed - first_worked);
                break;
            }
        }
    }

    println!(
        "The product of possible wins is {:?}",
        product.iter().fold(1, |acc, n| acc * n)
    );
}

// Haha big number make hard
pub fn pt2(input: String) {
    let [time_str, dist_str] = input.split("\r\n").collect::<Vec<&str>>()[..] else {
        panic!("Invalid input")
    };
    let time = time_str
        .split(" ")
        .filter(|s| !s.is_empty())
        .skip(1)
        .fold("".to_string(), |acc, t| acc + t)
        .parse::<i64>()
        .unwrap();
    let distance = dist_str
        .split(" ")
        .filter(|s| !s.is_empty())
        .skip(1)
        .fold("".to_string(), |acc, d| acc + d)
        .parse::<i64>()
        .unwrap();
    println!("T: {}, D: {}", time, distance);

    let mid = time / 2;
    println!("Score at midpoint: {}", (time - mid) * mid);
    assert!(
        (time - mid) * mid > distance,
        "The midpoint wasn't a winning score"
    );

    let mut endpoints: Vec<i64> = Vec::new();
    for (start_left, start_right, flip) in [(0, mid, false), (mid, time, true)] {
        let mut left = start_left;
        let mut right = start_right;
        while left <= right {
            let test = (left + right) / 2;
            let score = (time - test) * test;
            println!("L: {}, R: {}, T: {}, S: {}", left, right, test, score);
            if (score < distance) ^ flip {
                left = test + 1;
            } else if (score > distance) ^ flip {
                right = test - 1;
            } else {
                break;
            }
        }
        endpoints.push(if flip { right } else { left });
    }
    println!("Num wins: {}", endpoints[1]-endpoints[0]+1);
}

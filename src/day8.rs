use std::collections::HashMap;

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let mut lines = input.split("\r\n");
    let path = lines.next().unwrap();
    lines.next();
    let nodes: Vec<&str> = lines.collect();
    let mut connections: HashMap<&str, (&str, &str)> = HashMap::with_capacity(nodes.len());
    for node in nodes {
        let [name, conns] = node.split(" = (").collect::<Vec<&str>>()[..] else {
            panic!("Invalid input")
        };
        let [left, right] = conns
            .split(" ")
            .map(|c| c.strip_suffix(|_: char| true).unwrap())
            .collect::<Vec<&str>>()[..]
        else {
            panic!("Invalid input")
        };
        connections.insert(name, (left, right));
    }

    if pt1 {
        // Single position logic
        let mut steps = 0;
        let mut pos = "AAA";
        for step in path.chars().cycle() {
            steps += 1;
            let conns = connections.get(pos).expect("Invalid input");
            pos = if step == 'L' { conns.0 } else { conns.1 };
            if pos == "ZZZ" {
                break;
            }
        }
        println!("Steps taken: {steps}");
    } else {
        // Multi-position logic
        let posns: Vec<&str> = connections
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|&k| k)
            .collect();

        let p_steps: Vec<i64> = posns
            .iter()
            .map(|init_pos| {
                let mut steps = 0;
                let mut pos = init_pos.to_owned();
                for step in path.chars().cycle() {
                    steps += 1;
                    let conns = connections.get(pos).expect("Invalid input");
                    pos = if step == 'L' { conns.0 } else { conns.1 };
                    if pos.ends_with('Z') {
                        break;
                    }
                }
                steps
            })
            .collect();

        // This lcm trick only works when there's a cycle, but it seems that the inputs are
        // structured such that this is always the case.
        println!("Steps taken: {}", lcm(p_steps.as_slice()));
    }
}

fn find_next(vals: Vec<i32>) -> i32 {
    let mut diffs: Vec<i32> = Vec::new();
    for pair in vals.windows(2) {
        diffs.push(pair[1] - pair[0]);
    }
    if diffs.iter().all(|&d|d==0) {
        vals[0]
    } else {
        vals.last().unwrap() + find_next(diffs)
    }
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let lines = input.split("\r\n");

    let mut sum = 0;
    for line in lines {
        let mut nums: Vec<i32> = line.split(" ").map(|n|n.parse::<i32>().unwrap()).collect();
        if !pt1 {
            nums.reverse(); // [that was easy]
        }
        let next = find_next(nums.clone());
        println!("Nums: {nums:?}, Next: {next}");
        sum += next;
    }
    println!("Sum of next vals: {sum}");
}

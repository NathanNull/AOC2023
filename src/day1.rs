const NUM_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUM_VALS: [&str; 9] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

// Input is strings of text, find first and last numbers in them
// and form 2-digit numbers. Then add them.
pub fn pt1(input: String) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut nums: Vec<(usize, usize)> = Vec::new();

        for (i, name) in NUM_VALS.iter().enumerate()
        {
            for (idx, _) in line.match_indices(name) {
                nums.push((idx, i + 1));
            }
        }

        nums.sort_by_key(|k| k.0);
        let d1 = nums.first().unwrap().1;
        let d2 = nums.last().unwrap().1;
        sum += d1 * 10 + d2;
    }
    println!("Result is {}", sum);
}

// Exactly the same, but this time with spelled-out numbers too.
pub fn pt2(input: String) {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut nums: Vec<(usize, usize)> = Vec::new();

        for (i, name) in NUM_NAMES
            .iter()
            .enumerate()
            .chain(NUM_VALS.iter().enumerate())
        {
            for (idx, _) in line.match_indices(name) {
                nums.push((idx, i + 1));
            }
        }

        nums.sort_by_key(|k| k.0);
        let d1 = nums.first().unwrap().1;
        let d2 = nums.last().unwrap().1;
        sum += d1 * 10 + d2;
    }
    println!("Result is {}", sum);
}

fn get_nums(nums: &str) -> impl Iterator<Item = &str> {
    nums.split(" ").filter(|num| num.len() > 0)
}

pub fn pt1(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut points = 0;
    for line in lines {
        let ticket = line.split(": ").nth(1).unwrap();
        let nums: Vec<&str> = ticket.split(" | ").collect();
        let win_nums: Vec<&str> = get_nums(nums[1]).collect();
        let wins: Vec<&str> = get_nums(nums[0])
            .filter(|num| win_nums.contains(num))
            .collect();
        let win_count = wins.len() as i32;
        if win_count > 0 {
            points += 2_i32.pow((win_count - 1) as u32);
        }
    }
    println!("The cards are worth {} points in total.", points);
}

pub fn pt2(input: String) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut tickets: Vec<i32> = Vec::new();
    for _ in 0..lines.len() {
        tickets.push(1);
    }
    for (idx, line) in lines.iter().enumerate() {
        let ticket = line.split(": ").nth(1).unwrap();
        let nums: Vec<&str> = ticket.split(" | ").collect();
        let win_nums: Vec<&str> = get_nums(nums[1]).collect();
        let wins: Vec<&str> = get_nums(nums[0])
            .filter(|num| win_nums.contains(num))
            .collect();
        let win_count = wins.len() as i32;
        let curr_num_tickets = tickets[idx];
        if win_count > 0 {
            for win in 0..win_count as usize {
                tickets[idx + win + 1] += curr_num_tickets;
            }
        }
    }
    println!(
        "{} total scratchcards have been processed.",
        tickets.iter().fold(0, |acc, t| acc + t)
    );
}

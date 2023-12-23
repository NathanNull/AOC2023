pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn offset(&self) -> (i64, i64) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

//ans pt1 (for testing new approaches): 36725
//TODO: Look up what the shoelace formula is, it seems useful
// So's Pick's theorem apparently
// Some neat math to look at here I think

fn main(input: String, pt1: bool) {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let [dir, num, col] = line.split(' ').collect::<Vec<_>>()[..] else {
                panic!("Invalid input!")
            };
            let (mut d, mut n) = (dir.to_string(), num.to_string());
            if !pt1 {
                let mut cnum = col.split(['(', '#', ')']).nth(2).unwrap().chars();
                d = "RDLU"
                    .chars()
                    .nth(cnum.next_back().and_then(|c| c.to_digit(4)).unwrap() as usize)
                    .unwrap()
                    .to_string();
                n = cnum
                    .map(|c| c.to_digit(16).unwrap())
                    .fold(0, |acc, n| acc * 16 + n)
                    .to_string();
            }
            (
                Direction::from_char(d.chars().next().unwrap()),
                n.parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut points = Vec::new();
    let mut pos = (0, 0);
    points.push(pos);
    for (dir, num) in instructions {
        let (ox, oy) = dir.offset();
        pos = (pos.0 + ox * num, pos.1 + oy * num);
        points.push(pos);
    }

    // abs(shoelace sum) = total area * 2
    let mut shoelace_sum = 0;
    let mut border_points = 0;
    for pair in points.windows(2) {
        let [(x1, y1), (x2, y2)] = pair else {
            panic!("How even")
        };
        shoelace_sum += (x2 + x1) * (y2 - y1);
        border_points += i64::abs(y2 - y1) + i64::abs(x2 - x1);
    }
    // Abs for protection against going counterclockwise
    let area = i64::abs(shoelace_sum / 2);

    // Pick's Theorem
    // A = I + B/2 - 1
    // I = A - B/2 + 1
    let interior = area - border_points / 2 + 1;

    println!("Total filled: {}", border_points + interior);
}

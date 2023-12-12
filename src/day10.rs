use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Pipe {
        left: bool,
        up: bool,
        down: bool,
        right: bool,
    },
    Blank,
    Start,
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let pipes: Vec<Vec<Cell>> = input
        .split("\r\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '7' => Cell::Pipe {
                        left: true,
                        up: false,
                        down: true,
                        right: false,
                    },
                    'J' => Cell::Pipe {
                        left: true,
                        up: true,
                        down: false,
                        right: false,
                    },
                    'F' => Cell::Pipe {
                        left: false,
                        up: false,
                        down: true,
                        right: true,
                    },
                    'L' => Cell::Pipe {
                        left: false,
                        up: true,
                        down: false,
                        right: true,
                    },
                    '-' => Cell::Pipe {
                        left: true,
                        right: true,
                        up: false,
                        down: false,
                    },
                    '|' => Cell::Pipe {
                        up: true,
                        down: true,
                        left: false,
                        right: false,
                    },
                    'S' => Cell::Start,
                    '.' => Cell::Blank,
                    c => panic!("Invalid input! '{}'", c),
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    let row_with_idx = pipes
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(&Cell::Start))
        .unwrap();
    let pos = (
        row_with_idx.0,
        row_with_idx
            .1
            .iter()
            .enumerate()
            .find(|(_, &cell)| cell == Cell::Start)
            .unwrap()
            .0,
    ); // Wow this code is awful

    println!("{:?}", pos);

    let mut explored: HashMap<(usize, usize), i32> = HashMap::new();
    explored.insert(pos, 0);

    let mut to_check: VecDeque<(usize, usize, i32)> = VecDeque::new();

    to_check.push_back((pos.0, pos.1 + 1, 1));
    to_check.push_back((pos.0, pos.1 - 1, 1));
    to_check.push_back((pos.0 + 1, pos.1, 1));
    to_check.push_back((pos.0 - 1, pos.1, 1));

    while !to_check.is_empty() {
        let mut check = to_check.pop_front().unwrap();
        let Some(row) = pipes.get(check.0) else {continue};
        let Some(cell) = row.get(check.1) else {continue};
        match cell {
            Cell::Blank => (),
            Cell::Start => 
        }
    }

    
}

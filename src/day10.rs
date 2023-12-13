use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Pipe([Dir; 2]),
    Blank,
    Start,
}

// impl Cell {
//     fn _vis(&self) -> char {
//         match self {
//             Self::Blank => '.',
//             Self::Start => 'S',
//             Self::Pipe {
//                 left: true,
//                 up: false,
//                 down: true,
//                 right: false,
//             } => '7',
//             Self::Pipe {
//                 left: true,
//                 up: true,
//                 down: false,
//                 right: false,
//             } => 'J',
//             Self::Pipe {
//                 left: false,
//                 up: true,
//                 down: false,
//                 right: true,
//             } => 'L',
//             Self::Pipe {
//                 left: false,
//                 up: false,
//                 down: true,
//                 right: true,
//             } => 'F',
//             Self::Pipe {
//                 left: true,
//                 up: false,
//                 down: false,
//                 right: true,
//             } => '-',
//             Self::Pipe {
//                 left: false,
//                 up: true,
//                 down: true,
//                 right: false,
//             } => '|',
//             Self::Pipe { .. } => panic!("Invalid pipe type"),
//         }
//     }
// }

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, _pt1: bool) {
    let pipes: HashMap<(i32, i32), Cell> = input
        .split("\r\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (x as i32, y as i32),
                    match c {
                        '7' => Cell::Pipe([Dir::Down, Dir::Left]),
                        'J' => Cell::Pipe([Dir::Up, Dir::Left]),
                        'F' => Cell::Pipe([Dir::Right, Dir::Down]),
                        'L' => Cell::Pipe([Dir::Up, Dir::Right]),
                        '-' => Cell::Pipe([Dir::Right, Dir::Left]),
                        '|' => Cell::Pipe([Dir::Up, Dir::Down]),
                        'S' => Cell::Start,
                        '.' => Cell::Blank,
                        c => panic!("Invalid input! '{}'", c),
                    },
                )
            })
        })
        .flatten()
        .collect();

    let start_pos = *pipes
        .iter()
        .find(|(_, &cell)| cell == Cell::Start)
        .unwrap()
        .0;
    let mut pos = start_pos;
    let mut last_dir = Dir::Down;
    for t in [Dir::Left, Dir::Right, Dir::Down, Dir::Up] {
        let offset = t.offset();
        let (v,h) = (-offset.0, -offset.1);
        let to_check = (pos.0 as i32 + h, pos.1 as i32 + v);
        if to_check.0 < 0 || to_check.1 < 0 {
            continue;
        }
        match pipes
            .get(&to_check)
            .unwrap_or(&Cell::Blank)
        {
            Cell::Pipe(dirs) if dirs.contains(&t) => {
                pos = to_check;
                last_dir = t;
                println!("Pos: {pos:?}, dir: {t:?}");
                break;
            }
            _ => (),
        }
    }

    let mut steps = 1;
    while pos != start_pos {
        let cell = pipes.get(&pos).unwrap_or(&Cell::Blank);
        match cell {
            Cell::Pipe(dirs) => {
                let dir = dirs.iter().find(|&&d| d != last_dir).unwrap();
                let offset = dir.offset();
                pos = (pos.0+offset.0, pos.1+offset.1);
                last_dir = dir.opposite();
                steps += 1;
                println!("Pos: {pos:?}, dir: {dir:?}, offset: {offset:?}");
            }
            _ => panic!("Derailed"),
        }
    }
    println!("Took {steps} steps, furthest is {}", steps/2);
}

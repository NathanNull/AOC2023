use std::collections::{HashMap, HashSet};

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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    Pipe([Dir; 2]),
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
    let mut pipes: HashMap<(i32, i32), Cell> = input
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
                        _ => Cell::Blank, // Accept ., O, I, etc.
                        //c => panic!("Invalid input! '{}'", c),
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
        let (h, v) = (-offset.0, -offset.1);
        let to_check = (pos.0 as i32 + h, pos.1 as i32 + v);
        if to_check.0 < 0 || to_check.1 < 0 {
            continue;
        }
        match pipes.get(&to_check).unwrap_or(&Cell::Blank) {
            Cell::Pipe(dirs) if dirs.contains(&t) => {
                pos = to_check;
                last_dir = t;
                //println!("Pos: {pos:?}, dir: {t:?}");
                break;
            }
            _ => (),
        }
    }

    let s_first = last_dir.opposite();

    let mut steps = 1;
    let mut path: HashSet<(i32, i32)> = HashSet::new();
    path.insert(pos);
    while pos != start_pos {
        let cell = pipes.get(&pos).unwrap_or(&Cell::Blank);
        match cell {
            Cell::Pipe(dirs) => {
                let dir = dirs.iter().find(|&&d| d != last_dir).unwrap();
                let offset = dir.offset();
                pos = (pos.0 + offset.0, pos.1 + offset.1);
                last_dir = dir.opposite();
                steps += 1;
                path.insert(pos);
                //println!("Pos: {pos:?}, dir: {dir:?}, offset: {offset:?}");
            }
            _ => panic!("Derailed"),
        }
    }

    let s_second = last_dir;
    pipes.insert(start_pos, Cell::Pipe([s_first, s_second]));

    if pt1 {
        println!("Took {steps} steps, furthest is {}", steps / 2);
        return;
    }

    // P A I N
    let mut enclosed = 0;
    for y in 0..*pipes.keys().map(|(_, y)| y).max().unwrap() + 1 {
        let mut outside = true;
        let mut on_line = false;
        let mut started_up = false;
        let mut row: Vec<(&(i32, i32), &Cell)> =
            pipes.iter().filter(|(&(_, ty), _)| ty == y).collect();
        row.sort_by_key(|((x, _), _)| x);
        for (pos, c) in row {
            if !path.contains(pos) {
                // Treat it like it's a blank
                if !outside {
                    enclosed += 1;
                    print!("{:?}, ", pos);
                }
                continue;
            }
            match c {
                Cell::Blank if !outside => {
                    enclosed += 1;
                    print!("{:?}, ", pos);
                },
                Cell::Pipe(dirs) if dirs.contains(&Dir::Up) || dirs.contains(&Dir::Down) => {
                    if on_line {
                        // This means either J or 7.
                        on_line = false;
                        if started_up == dirs.contains(&Dir::Down) {
                            // If L---7 or F---J
                            outside = !outside;
                        }
                    } else if dirs.contains(&Dir::Right) {
                        // This means either L or F
                        on_line = true;
                        started_up = dirs.contains(&Dir::Up);
                    } else {
                        // This means |
                        outside = !outside;
                    }
                }
                _ => (),
            }
        }
    }

    println!("\nInside pieces found: {}", enclosed)
}

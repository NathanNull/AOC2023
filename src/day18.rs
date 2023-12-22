use std::collections::{HashSet, VecDeque};

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

    fn offset(&self) -> (i32, i32) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }

    fn all() -> Vec<Self> {
        vec![Up,Down,Left,Right]
    }
}

//ans pt1 (for testing new approaches): 36725
//TODO: Look up what the shoelace formula is, it seems useful
// So's Pick's theorem apparently
// Some neat math to look at here I think

fn main(input: String, pt1: bool) {
    let instructions: Vec<(Direction, usize, &str)> = input
        .lines()
        .map(|line| {
            let [dir, num, col] = line.split(' ').collect::<Vec<_>>()[..] else {
                panic!("Invalid input!")
            };
            (
                Direction::from_char(dir.chars().next().unwrap()),
                num.parse::<usize>().unwrap(),
                // Why the color is here I really don't know
                col.split(['(', '#', ')']).nth(2).unwrap(),
            )
        })
        .collect();

    let mut dug = HashSet::new();
    let mut pos = (0, 0);
    dug.insert(pos);
    for (dir, num, col) in instructions {
        let offset = dir.offset();
        for _ in 0..num {
            pos = (pos.0 + offset.0, pos.1 + offset.1);
            dug.insert(pos);
        }
    }

    let min_x = dug.iter().min_by_key(|p|p.0).unwrap().0;
    let min_y = dug.iter().min_by_key(|p|p.1).unwrap().1;
    let max_x = dug.iter().max_by_key(|p|p.0).unwrap().0;
    let max_y = dug.iter().max_by_key(|p|p.1).unwrap().1;
    dug = HashSet::from_iter(dug.iter().map(|(dx,dy)|(dx-min_x,dy-min_y)));
    let mut ground = Vec::new();
    let mut to_check = Vec::new();
    for y in min_y..max_y+1 {
        ground.push(Vec::new());
        for x in min_y..max_x+1 {
            // Map from -x..+y to 0..+x+y
            let off_p = (x-min_x, y-min_y);
            let is_dug = dug.contains(&off_p);
            ground[off_p.1 as usize].push(is_dug);
            if !is_dug {
                to_check.push(off_p);
            }
            //print!("{}", if is_dug {'#'} else {'.'});
        }
        //println!();
    }

    let (bx,by) = (ground[0].len() as i32, ground.len() as i32);
    //println!("{} by {} = {}", bx,by,bx*by);
    

    // Brute-force floodfill approach
    // TODO: Use the better, formula-based way
    while let Some(curr) = to_check.pop() {
        let mut seen = HashSet::new();
        let mut open = VecDeque::new();
        let mut hit_outside = false;
        open.push_back(curr);

        // Flood fill type thing
        while let Some(check) = open.pop_front() {
            if seen.contains(&check) || dug.contains(&check) {
                continue;
            } else if check.0<0 || check.1<0 || check.0>bx || check.1>by {
                hit_outside = true;
                continue;
            }
            seen.insert(check);
            for neighbour in Direction::all().iter().map(|d|{
                let o = d.offset();
                (check.0+o.0, check.1+o.1)
            }) {
                open.push_back(neighbour);
            }
        }

        // As long as everything was contained, dig it out
        if !hit_outside {
            for seen_tile in &seen {
                dug.insert(*seen_tile);
            }
        }

        // If we've seen a tile, take it out of the list
        to_check.retain(|t| !seen.contains(t));
    }

    // for y in 0..max_y+1 {
    //     for x in 0..max_x+1 {
    //         print!("{}", if dug.contains(&(x,y)) {'#'} else {'.'});
    //     }
    //     println!();
    // }

    println!("Num tiles dug: {}", dug.len());
}

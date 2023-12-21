use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;
impl Direction {
    fn iter() -> Vec<Self> {
        vec![Up, Down, Left, Right]
    }
    fn from_offset(offset: (i32, i32)) -> Self {
        match offset {
            (1, 0) => Right,
            (-1, 0) => Left,
            (0, 1) => Down,
            (0, -1) => Up,
            _ => panic!("Invalid offset"),
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    fn index(&self) -> usize {
        match self {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Crucible {
    pos: usize,
    dir: Option<Direction>,
    moves: usize,
    cost: usize,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

// really irritating neighbours function
// fn neighbours(cruc: Crucible, block_size: (i32, i32)) -> Vec<Crucible> {
//     let b = cruc.pos;
//     return [(1, 0), (-1, 0), (0, 1), (0, -1)]
//         .iter()
//         .map(|&t| {
//             let dir = Direction::from_offset(t);
//             let moves = if dir == cruc.dir {cruc.moves + 1} else { 1 };
//             Crucible{pos: (b.0+t.0, b.1+t.1), dir, moves}
//         })
//         .filter(|t| t.pos.0 >= 0 && t.pos.0 <= block_size.0 && t.pos.1 >= 0 && t.pos.1 <= block_size.1)
//         .filter(|c|c.moves <= 3)
//         .collect();
// }

// Djikstra's alg. to the rescue
// (extra funny because technically 4d)
fn main(input: String, pt1: bool) {
    let blocks: Vec<Vec<usize>> = input
        .split("\r\n")
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let block_size = (blocks[0].len() - 1, blocks.len() - 1);

    let endpoint = block_size;
    let start = Crucible {
        pos: 0,
        dir: None,
        moves: 0,
        cost: 0,
    };
    let min_dist = if pt1 { 0 } else { 4 };
    let max_dist = if pt1 { 3 } else { 10 };

    let mut to_check = BinaryHeap::<Crucible>::new();
    // holds (visited, dist)
    let mut history = vec![(false, usize::MAX); block_size.0 * block_size.1 * 4 * max_dist];
    to_check.push(start);
    while let Some(Crucible {
        pos,
        dir,
        moves,
        cost,
    }) = to_check.pop()
    {
        match dir {
            Some(d) => history[pos * 4 * max_dist + d.index() * max_dist + moves].0 = true,
            None => {
                for d in 0..4 {
                    history[pos * 4 * max_dist + d * max_dist + moves].0 = true;
                }
            }
        }
        to_check.extend([Up, Right, Down, Left].iter().filter_map(|&d| {
            let (same, opp) = match dir {
                Some(pdir) => (pdir == d, pdir.opposite() == d),
                None => (true, false),
            };

            if (moves < min_dist && !same)
                || (moves > max_dist - 1 && same)
                || opp
                || match d {
                    Up => pos < block_size.0,
                    Right => pos % block_size.0 == block_size.0 - 1,
                    Down => pos / block_size.0 == block_size.1 - 1,
                    Left => pos % block_size.0 == 0,
                }
            {
                return None;
            }

            let npos = match d {
                Up => pos - block_size.0,
                Right => pos + 1,
                Down => pos + block_size.0,
                Left => pos - 1,
            };
            let nmoves = 1 + if same { moves } else { 0 };
            let nkey = npos * 4 * max_dist + d.index() * max_dist + nmoves;
            let ncost = cost + blocks[npos / block_size.0][npos % block_size.0];
            let (visited, prev_cost) = history[nkey];
            if visited || prev_cost <= ncost {
                return None;
            }
            history[nkey].1 = ncost;
            Some(Crucible {
                pos: npos,
                dir: Some(d),
                moves: nmoves,
                cost: ncost,
            })
        }));
    }

    // why is this one higherrrrrr
    // it's not an off-by-one, either, taking one off doesn't fix anything
    // I have the wrong path somehow
    println!(
        "Min cost is {}",
        history[(block_size.0*block_size.1 - 1) * 4 * max_dist..]
            .iter()
            .map(|(_visited, cost)| *cost)
            .min()
            .unwrap()-1
    )
}

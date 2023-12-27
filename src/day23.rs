use std::collections::{HashMap, VecDeque};

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn dirs(tile: char, pt1: bool) -> Vec<(i32, i32)> {
    match tile {
        '#' => vec![],
        c if c == '.' || !pt1 => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        '>' => vec![(1, 0)],
        '<' => vec![(-1, 0)],
        'v' => vec![(0, 1)],
        '^' => vec![(0, -1)],
        _ => panic!("Invalid tile"),
    }
}

//pt2: 4618 too low (and so is any other wrong guess, unless I really mess something up)

fn main(input: String, pt1: bool) {
    let tiles: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    // If the graph has no loops, as it does in pt1, a brute-force solution is quick enough.
    // But the solution from part 2 should work for it too, and it's faster

    let start = (1, 0);
    let end = (tiles[0].len() as i32 - 2, tiles.len() as i32 - 1);

    let mut intersections: HashMap<(i32, i32), HashMap<(i32, i32), i32>> = HashMap::new();
    // Should store (origin intersect, last, current, steps taken)
    let mut searching = VecDeque::new();
    // Require that the start and end are intersections, so they'll be counted as nodes on the graph
    intersections.insert(start, HashMap::new());
    intersections.insert(end, HashMap::new());

    // Begin the search
    searching.push_back((start, start, (1, 1), 1));
    while let Some((from, last, curr, steps)) = searching.pop_front() {
        let curr_tile = tiles[curr.1 as usize][curr.0 as usize];
        let found_dirs: Vec<_> = dirs(curr_tile, pt1)
            .iter()
            .map(|(ox, oy)| (curr.0 + ox, curr.1 + oy))
            .filter(|&(nx, ny)| {
                // As long as it's not a wall
                tiles
                    .get(ny as usize)
                    .unwrap_or(&vec![])
                    .get(nx as usize)
                    .unwrap_or(&'#')
                    != &'#'
            })
            .collect();
        // If it's not just a straight line or turn, it's new
        let is_new = found_dirs.iter().filter(|p| **p != last).count() > 1;
        let nfrom = if is_new { curr } else { from };
        let nsteps = if is_new { 1 } else { steps + 1 };
        if is_new {
            intersections.insert(curr, HashMap::new());
            intersections.get_mut(&from).unwrap().insert(curr, steps);
        }
        for ncurr in found_dirs {
            if ncurr == last && !is_new {
                // Ignore it if it's a reversal,
                // unless it's a new intersection
            } else if !intersections.contains_key(&ncurr) {
                searching.push_back((nfrom, curr, ncurr, nsteps));
            } else {
                // There's a path between from and ncurr that's nsteps long
                intersections.get_mut(&from).unwrap().insert(ncurr, nsteps);
            }
        }
    }

    println!(
        "Found {} intersections with average {} links\n...this may take a while",
        intersections.len(),
        intersections
            .iter()
            .map(|(_, v)| v.len() as f64)
            .sum::<f64>()
            / intersections.len() as f64
    );

    // Genuinely the stupidest brute force algorithm you've ever seen
    // ...but it works.
    let mut paths = VecDeque::new();
    paths.push_back((vec![start], 0));
    let mut longest = vec![];
    let mut l_len = 0;
    let mut found = 0;
    while let Some((path, len)) = paths.pop_front() {
        let pos = path.last().unwrap();
        for (npos, dist) in intersections.get(pos).unwrap() {
            if path.iter().any(|p| p == npos) {
                // No backtracking allowed
                continue;
            }
            let mut npath = path.clone();
            npath.push(*npos);
            if *npos == end {
                let tot_len = len + dist;
                if tot_len <= l_len {
                    //nothing!
                    //this might make it faster or something idk
                } else {
                    longest = npath.clone();
                    l_len = tot_len;
                }
                found += 1;
                if found % 100_000 == 0 {
                    println!(
                        "Found {}th path, so far longest is {}",
                        found,
                        l_len
                    );
                    println!("Paths being searched: {}", paths.len());
                    assert!(npath
                        .iter()
                        .all(|p| npath.iter().filter(|q| *q == p).count() == 1))
                }
            }
            paths.push_back((npath, len + dist));
        }
    }

    println!(
        "Longest path: {:?}\nLength={}",
        longest, l_len
    );
}

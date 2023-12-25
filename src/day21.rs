use std::collections::HashSet;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn step(grid: &Vec<Vec<char>>, reached: HashSet<(usize, usize)>, pt1: bool) -> HashSet<(usize, usize)> {
    let to_explore: Vec<_> = reached.iter().collect();
    let mut explored = HashSet::new();
    for curr in to_explore {
        for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (curr.0 as i32 + ox, curr.1 as i32 + oy);
            if pt1 && (nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32) {
                continue;
            }
            let (ux, uy) = (nx as usize % grid[0].len(), ny as usize % grid.len());
            if grid[uy][ux] != '#' {
                explored.insert((ux, uy));
            }
        }
    }
    explored
}

fn main(input: String, pt1: bool) {
    // technically this solution works, as is a valid claim
    // with any brute force solution. In any case, too tired
    // to find anything better until tomorrow maybe
    
    let total_steps: usize = if pt1 {64} else {26501365};
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // That sure is one way to find the coords of the S.
    let spos = grid
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .flatten()
        .find(|(_, c)| c == &&'S')
        .unwrap()
        .0;

    let mut reached = HashSet::new();
    reached.insert(spos);
    for _ in 0..total_steps {
        reached = step(&grid, reached, pt1);
    }
    println!("Reached {} plots", reached.len());
}

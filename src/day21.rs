use std::collections::HashSet;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn step(grid: &Vec<Vec<char>>, reached: HashSet<(i64, i64)>, pt1: bool) -> HashSet<(i64, i64)> {
    let to_explore: Vec<_> = reached.iter().collect();
    let mut explored = HashSet::new();
    for curr in to_explore {
        for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (curr.0 as i64 + ox, curr.1 as i64 + oy);
            if pt1 && (nx < 0 || ny < 0 || nx >= grid[0].len() as i64 || ny >= grid.len() as i64) {
                println!("Skipped a node");
                continue;
            }
            let (ux, uy) = (nx as usize % grid[0].len(), ny as usize % grid.len());
            if grid[uy][ux] != '#' {
                explored.insert((nx, ny));
            }
        }
    }
    explored
}

fn take_steps(grid: &Vec<Vec<char>>, start: (i64, i64), steps: usize, pt1: bool) -> usize {
    let mut reached = HashSet::new();
    reached.insert(start);
    for _ in 0..steps {
        reached = step(&grid, reached, pt1);
    }
    reached.len()
}

fn main(input: String, pt1: bool) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // That sure is one way to find the coords of the S.
    let u_spos = grid
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .flatten()
        .find(|(_, c)| c == &&'S')
        .unwrap()
        .0;
    let spos = (u_spos.0 as i64, u_spos.1 as i64);

    if pt1 {
        println!("Reached {} plots", take_steps(&grid, spos, 64, true));
        return;
    }

    let y0 = take_steps(&grid, spos, 65, false);
    let y1 = take_steps(&grid, spos, 65+131, false);
    let y2 = take_steps(&grid, spos, 65+131*2, false);
    let a2 = y2 - 2*y1 + y0;
    let b2 = 4*y1 - 3*y0 - y2;
    let c = y0;

    println!("{a2}/2 x^2 +{b2}/2 x + {c} = y");
    println!("x=0, y={c}");
    println!("x=1, y={}", (a2 + b2) / 2 + c);
    println!("x=2, y={}", (4 * a2 + 2 * b2) / 2 + c);
    println!(
        "x=202300, y={}",
        (202_300 * 202_300 * a2 + 202_300 * b2) / 2 + c
    );
}
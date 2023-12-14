use std::cmp::{min, max};

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let lines = input.split("\r\n");
    let spaces: Vec<Vec<bool>> = lines
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    let mut ex_rows: Vec<usize> = Vec::new();
    for (y,row) in spaces.iter().enumerate() {
        if row.iter().all(|r| !r) {
            ex_rows.push(y);
        }
    }

    let mut ex_cols: Vec<usize> = Vec::new();
    for x in 0..spaces[0].len() {
        if spaces.iter().all(|row| !row[x]) {
            ex_cols.push(x);
        }
    }

    let galaxies: Vec<(usize, usize)> = spaces
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .flatten()
        .filter(|(_, &c)| c)
        .map(|(pos, _)| pos)
        .collect(); // one hell of a list comprehension (or whatever you call these in Rust)

    let mut sum_paths = 0;
    let exp_mult = (if pt1 {2_i64} else {1_000_000_i64})-1;
    for (id, galaxy) in galaxies.iter().enumerate() {
        for (_, partner) in galaxies.iter().enumerate().filter(|&(p_id, _)| p_id > id) {
            let min_x = min(galaxy.0, partner.0);
            let min_y = min(galaxy.1, partner.1);
            let max_x = max(galaxy.0, partner.0);
            let max_y = max(galaxy.1, partner.1);
            let num_expanded_rows = ex_rows.iter().filter(|&&r|r>min_y && r<max_y).count() as i64;
            let num_expanded_cols = ex_cols.iter().filter(|&&c|c>min_x && c<max_x).count() as i64;
            sum_paths += (max_x-min_x) as i64+(max_y-min_y) as i64+(num_expanded_cols+num_expanded_rows)*exp_mult;
        }
    }

    println!("Sum of paths is {sum_paths}");
}

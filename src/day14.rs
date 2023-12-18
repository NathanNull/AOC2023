pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn tilt(cells: Vec<Vec<char>>, use_cols: bool, flip: bool) -> Vec<Vec<char>> {
    let mut to_use: Vec<Vec<char>> = if use_cols {
        cells[0]
            .iter()
            .enumerate()
            .map(|(i, _)| cells.iter().map(|g| g[i]).collect())
            .collect()
    } else {
        cells
    };

    if flip {
        to_use = to_use
            .iter()
            .map(|row| row.iter().rev().map(|&c| c).collect())
            .collect()
    }

    let mut new_cells: Vec<Vec<char>> = Vec::new();
    for row in to_use {
        let mut new_row: Vec<char> = Vec::new();
        let mut blanks = 0;
        let mut rocks = 0;
        for cell in row {
            match cell {
                '#' => {
                    for _ in 0..blanks {
                        new_row.push('.');
                    }
                    for _ in 0..rocks {
                        new_row.push('O');
                    }
                    new_row.push('#');
                    blanks = 0;
                    rocks = 0;
                }
                '.' => blanks += 1,
                'O' => rocks += 1,
                _ => panic!("what is this character"),
            }
        }
        for _ in 0..blanks {
            new_row.push('.');
        }
        for _ in 0..rocks {
            new_row.push('O');
        }
        new_cells.push(new_row);
    }

    if flip {
        new_cells = new_cells
            .iter()
            .map(|row| row.iter().rev().map(|&c| c).collect())
            .collect()
    }

    if use_cols {
        new_cells[0]
            .iter()
            .enumerate()
            .map(|(i, _)| new_cells.iter().map(|g| g[i]).collect())
            .collect()
    } else {
        new_cells
    }
}

// true, true == north
// true, false == south
// false, true == west
// false, false == east
fn cycle(cells: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = tilt(cells, true, true);
    let w = tilt(n, false, true);
    let s = tilt(w, true, false);
    let e = tilt(s, false, false);
    e
}

fn main(input: String, pt1: bool) {
    let mut grid = input
        .split("\r\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if pt1 {
        grid = tilt(grid, true, true);
    } else {
        let mut cycles = 0;
        let mut seen: Vec<Vec<Vec<char>>> = Vec::new();
        while !seen.contains(&grid) {
            seen.push(grid.clone());
            grid = cycle(grid);
            cycles += 1;
            // println!(
            //     "{}\n\n###################################\n",
            //     grid
            //         .iter()
            //         .map(|r| format!("{:?}", r))
            //         .collect::<Vec<_>>()
            //         .join("\r\n")
            // );
        }
        let first_idx = seen
            .iter()
            .enumerate()
            .find(|(_, s)| s == &&grid)
            .unwrap()
            .0;
        let cycle_len = cycles - first_idx;
        println!("First found at {first_idx} (now {cycles}), cycle length is {cycle_len}");
        let max_idx = 1_000_000_000;
        let skipped = (max_idx - cycles) / cycle_len;
        cycles += skipped * cycle_len;
        println!("Skipped {skipped} loops, now at {cycles}");
        while cycles != max_idx {
            cycles += 1;
            grid = cycle(grid);
        }
    }

    // println!(
    //     "{}\n\n###################################\n",
    //     grid
    //         .iter()
    //         .map(|r| format!("{:?}", r))
    //         .collect::<Vec<_>>()
    //         .join("\r\n")
    // );

    let load = grid
        .iter()
        .rev()
        .enumerate()
        .map(|(i, col)| col.iter().filter(|&&c| c == 'O').map(move |_| i + 1))
        .flatten()
        .fold(0, |a, b| a + b);
    println!("load: {load}")
}

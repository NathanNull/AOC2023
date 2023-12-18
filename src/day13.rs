use std::iter::zip;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn find_symmetric_row(cells: Vec<Vec<impl Eq>>, smudges: usize) -> i32 {
    for row in 1..cells.len() as i32 {
        let mut diff_found = 0;
        for offset in 0..i32::MAX {
            let Some(cr) = cells.get(offset as usize) else {
                break;
            };
            let Some(rev_offset): Option<usize> = (row * 2 - offset - 1).try_into().ok()
            // -(offset-row)+row-1
            // Good old off-by-one
            else {
                continue;
            };
            let Some(rr) = cells.get(rev_offset) else {
                continue;
            };
            diff_found += zip(cr, rr).filter(|(c, r)| c != r).count();
            if offset > 100 {
                panic!("hmm");
            }
        }
        if diff_found == 2 * smudges {
            return row;
        }
    }
    return 0;
}

fn main(input: String, pt1: bool) {
    let patterns = input.split("\r\n\r\n");
    let mut sum = 0;
    for pattern in patterns {
        println!("{pattern}");
        let cells = pattern
            .split("\r\n")
            .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = cells.iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>();
        let v_cells: Vec<Vec<bool>> = cells[0]
            .iter()
            .enumerate()
            .map(|(i, _)| rows.iter().map(|&r| cells[r][i]).collect())
            .collect();
        let smudges = if pt1 {0} else {1};
        let rows = find_symmetric_row(cells, smudges);
        let cols = find_symmetric_row(v_cells, smudges);
        println!("Valid rows: {rows}, cols: {cols}");
        sum += cols + rows * 100;
    }
    println!("Final score: {sum}");
    // do pt2 later
}

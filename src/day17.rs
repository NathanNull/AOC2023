use std::collections::HashMap;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn neighbours(b: (i32, i32), block_size: (i32, i32)) -> Vec<(i32, i32)> {
    return [
        (b.0 + 1, b.1),
        (b.0 - 1, b.1),
        (b.0, b.1 + 1),
        (b.0, b.1 - 1),
    ]
    .iter()
    .filter(|t| t.0 >= 0 && t.0 <= block_size.0 && t.1 >= 0 && t.1 <= block_size.1)
    .map(|&t| t)
    .collect();
}

// Possibly A* isn't the ideal choice here, IDK
fn main(input: String, pt1: bool) {
    let blocks: Vec<Vec<i32>> = input
        .split("\r\n")
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let block_ends = (blocks[0].len() as i32 - 1, blocks.len() as i32 - 1);
    let avg = blocks.iter().flatten().sum::<i32>() / (block_ends.0+1) / (block_ends.1+1);

    let endpoint = block_ends;
    // Basic Manhattan distance from endpoint, times average distance per block
    let h = |tile: (i32, i32)| (i32::abs(tile.0-endpoint.0) + i32::abs(tile.1-endpoint.1)) * avg;

    let mut to_check: Vec<(i32, i32)> = Vec::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    // stores (g,f) for every tile
    let mut scores: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    to_check.push((0, 0));
    scores.insert((0, 0), (0, h((0,0))));

    let mut completed = false;
    while !to_check.is_empty() {
        to_check.sort_by_key(|tile| {
            let t_scores = scores.get(tile).unwrap();
            -(t_scores.1*500-t_scores.0)
        });
        println!("To check: {:?}", to_check.iter().map(|t|(t,scores.get(t).unwrap())).collect::<Vec<_>>());
        let curr = to_check.pop().unwrap();
        println!("Checking {curr:?}");
        if curr == endpoint {
            completed = true;
            break;
        }

        for n in neighbours(curr, block_ends) {
            let trial_g = scores.get(&curr).unwrap().0 + blocks[n.1 as usize][n.0 as usize];
            if scores.get(&n).unwrap_or(&(i32::MAX, i32::MAX)).0 > trial_g {
                came_from.insert(n, curr);
                scores.insert(n, (trial_g, trial_g+h(n)));
            }
            if !to_check.iter().any(|block|block==&n) {
                to_check.push(n);
            }
        }
    }
    if completed {
        let mut path = Vec::new();
        let mut curr = endpoint;
        let mut heat_loss = blocks[curr.1 as usize][curr.0 as usize];
        path.push(curr);
        while came_from.contains_key(&curr) {
            curr = *came_from.get(&curr).unwrap();
            path.push(curr);
            heat_loss += blocks[curr.1 as usize][curr.0 as usize];
        }
        path.reverse();
        println!("Path is {:?}, heat loss is {}", path, heat_loss);
    }
}

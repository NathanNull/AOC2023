fn without_first<'a>(
    iter: &'a mut dyn Iterator<Item = &'a str>,
) -> &'a mut dyn Iterator<Item = &'a str> {
    iter.next();
    iter
}

pub fn pt1(input: String) {
    let mut maps = input.split("\r\n\r\n");
    let mut map_data = maps.next().unwrap().split(" ");
    let mut seeds: Vec<i64> = without_first(&mut map_data)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for map in maps {
        let mut range_data = map.split("\r\n");
        let ranges: Vec<&str> = without_first(&mut range_data).collect();
        let mut new_seeds = seeds.clone();
        for range in ranges {
            let data: Vec<i64> = range
                .split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let [dest, source, len] = data[..] else {
                panic!("Invalid map data")
            };
            let end = source + len - 1;
            for (idx, seed) in seeds.iter().enumerate() {
                if seed >= &source && seed <= &end {
                    new_seeds[idx] = dest + (seed - source);
                }
            }
        }
        seeds = new_seeds;
    }

    seeds.sort();
    println!("The lowest seed result is {}", seeds[0]);
}

// god I hate range math
pub fn pt2(input: String) {
    let mut maps = input.split("\r\n\r\n");
    let mut map_data = maps.next().unwrap().split(" ");
    let mut seeds: Vec<[i64; 2]> = without_first(&mut map_data)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|c| [c[0], c[0] + c[1] - 1])
        .collect();
    for map in maps {
        let mut range_data = map.split("\r\n");
        let ranges: Vec<&str> = without_first(&mut range_data).collect();
        let mut new_seeds = seeds.clone();
        let mut moved: Vec<[i64; 2]> = Vec::new();
        for range in ranges {
            let data: Vec<i64> = range
                .split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let [dest, start, len] = data[..] else {
                panic!("Invalid map data")
            };
            let end = start + len - 1;
            for (idx, &[s_start, s_end]) in new_seeds.clone().iter().enumerate() {
                if s_start > end || s_end < start {
                    continue;
                } // There's no overlap here
                match (s_start >= start, s_end <= end) {
                    (true, true) => {
                        moved.push([dest + (s_start - start), dest + (s_end - start)]);
                        new_seeds[idx] = [-1, -1];
                        // ^^ Take the element out, as it's been moved entirely
                    }
                    (true, false) => {
                        new_seeds[idx] = [end + 1, s_end];
                        moved.push([dest + (s_start - start), dest + len - 1]);
                    }
                    (false, true) => {
                        new_seeds[idx] = [s_start, start - 1];
                        moved.push([dest, dest + (s_end - start)]);
                    }
                    (false, false) => {
                        new_seeds[idx] = [s_start, start - 1];
                        moved.push([dest, dest + len - 1]);
                        new_seeds.push([end + 1, s_end]);
                    }
                }
            }
        }
        new_seeds.append(&mut moved);
        new_seeds.retain(|&s|s != [-1, -1]);
        seeds = new_seeds;
    }

    seeds.sort();
    println!("The lowest seed result is {}", seeds[0][0]);
}

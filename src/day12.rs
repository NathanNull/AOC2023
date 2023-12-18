use std::collections::HashMap;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Record {
    Damaged,
    Safe,
    Unknown,
}

//caching is nice
fn count_ways(record: Vec<Record>, groups: Vec<usize>) -> usize {
    let mut cache = HashMap::new();
    cw_raw(record, groups, 0, &mut cache)
}

type CWCache = HashMap<(Vec<Record>, Vec<usize>, usize), usize>;
fn cw_raw(record: Vec<Record>, groups: Vec<usize>, num_done: usize, cache: &mut CWCache) -> usize {
    let key = (record.clone(), groups.clone(), num_done);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let res = cw_inner(record, groups, num_done, cache);
    cache.insert(key, res);
    return res;
}

fn cw_inner(
    record: Vec<Record>,
    groups: Vec<usize>,
    num_done: usize,
    cache: &mut CWCache,
) -> usize {
    if record.is_empty() {
        return if groups.is_empty() && num_done == 0 {
            1
        } else {
            0
        };
    }
    let mut solutions = 0;
    for allowed in if record[0] == Record::Unknown {
        vec![Record::Safe, Record::Damaged]
    } else {
        vec![record[0]]
    } {
        if allowed == Record::Damaged {
            solutions += cw_raw(record[1..].to_vec(), groups.clone(), num_done + 1, cache);
        } else if num_done != 0 {
            if !groups.is_empty() && groups[0] == num_done {
                solutions += cw_raw(record[1..].to_vec(), groups[1..].to_vec(), 0, cache);
            }
        } else {
            solutions += cw_raw(record[1..].to_vec(), groups.clone(), 0, cache);
        }
    }
    solutions
}

fn join_copies<'a>(base: &'a str, num: usize, sep: &'a str) -> String {
    [base]
        .iter()
        .cycle()
        .take(num)
        .map(|&s| s)
        .collect::<Vec<_>>()
        .join(sep)
    // One heck of a one-liner
}

fn main(input: String, pt1: bool) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut sum = 0;
    for line in lines {
        let info: Vec<_> = line.split(" ").collect();
        let [r_info, g_info]: [&str] = info[..] else {
            panic!("Invalid input!")
        };
        let record: Vec<Record> = join_copies(r_info, if pt1 { 1 } else { 5 }, "?")
            .chars()
            .map(|c| match c {
                '#' => Record::Damaged,
                '.' => Record::Safe,
                '?' => Record::Unknown,
                _ => panic!("Invalid input!"),
            })
            .chain([Record::Safe].iter().map(|&r| r))
            .collect();
        let group_counts: Vec<usize> = join_copies(g_info, if pt1 { 1 } else { 5 }, ",")
            .split(",")
            .map(|g| g.parse::<usize>().unwrap())
            .collect();
        let num_ways = count_ways(record, group_counts);
        //println!("Ways=={}", num_ways);
        sum += num_ways;
    }
    println!("Total sum: {sum}");
}

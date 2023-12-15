pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Record {
    Damaged,
    Safe,
    Unknown,
}

fn count_ways(record: Vec<Record>, groups: Vec<usize>) -> Vec<Vec<usize>> {
    let Some(&first_len) = groups.first() else {
        let mut ret: Vec<Vec<usize>> = Vec::new();
        if record.contains(&Record::Damaged) {
            return ret;
        }
        ret.push(Vec::new());
        return ret;
    };

    let mut start_posns: Vec<usize> = Vec::new();
    let mut must_start = false;
    let mut could_break = false;
    println!("Len: {first_len}, Record: {record:?}");
    for (pos, record_pipe) in record.iter().enumerate() {
        let curr_valid: usize = (pos as i32 - first_len as i32).try_into().unwrap_or(0);
        match record_pipe {
            Record::Safe => {
                start_posns = start_posns
                    .iter()
                    .filter(|&&spos| spos <= curr_valid)
                    .map(|&s| s)
                    .collect();
                if must_start {
                    break;
                }
            }
            Record::Damaged => {
                must_start = true;
                start_posns.push(pos);
                start_posns = start_posns
                    .iter()
                    .filter(|&&spos| spos != curr_valid)
                    .map(|&s| s)
                    .collect();
            }
            Record::Unknown => {
                //Nothing can be ruled out by the existence of a ?
                start_posns.push(pos);
                if must_start {
                    if !could_break {
                        start_posns = start_posns
                            .iter()
                            .filter(|&&spos| spos != curr_valid)
                            .map(|&s| s)
                            .collect();
                        could_break = true;
                        must_start = false;
                    }
                }
            }
        }
        println!("Checked: {record_pipe:?}");
        println!("Start positions: {start_posns:?}");
    }
    println!("----Stage 2: {start_posns:?}----");

    let mut ways: Vec<Vec<usize>> = Vec::new();
    let mut next_groups = groups.clone();
    next_groups.remove(0);
    for spos in start_posns {
        println!("Considering: {spos}");
        let rec_after = *record.get(spos + first_len).unwrap_or(&Record::Damaged);
        println!("Char after: {rec_after:?}");
        if rec_after != Record::Damaged {
            let mut next_record = record.clone();

            for _ in 0..spos + first_len {
                next_record.remove(0);
            }
            next_record[0] = Record::Safe;
            println!("{next_record:?}");
            println!("-------Going deeper--------");
            ways.extend(
                count_ways(next_record, next_groups.clone())
                    .iter()
                    .map(|s| [spos].iter().chain(s.iter()).map(|&s| s).collect())
                    .collect::<Vec<_>>(),
            );
            println!("-------Done--------");
        }
    }
    ways
}

fn main(input: String, pt1: bool) {
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut sum = 0;
    for line in lines{//}.iter().rev().take(1) {
        let mut info = line.split(" ");
        let record: Vec<Record> = info
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => Record::Damaged,
                '.' => Record::Safe,
                '?' => Record::Unknown,
                _ => panic!("Invalid input!"),
            })
            .chain([Record::Safe].iter().map(|&s| s))
            .collect();
        let group_counts: Vec<usize> = info
            .next()
            .unwrap()
            .split(",")
            .map(|g| g.parse::<usize>().unwrap())
            .collect();
        let num_ways = count_ways(record, group_counts);
        println!("################################  Done  ###################################");
        println!("Ways=={}", num_ways.len());
        sum += num_ways.len();
    }
    println!("Total sum: {sum}");
    // Looks like #3 (1,3,1,6) is broken, not sure why. Debug it later.
}

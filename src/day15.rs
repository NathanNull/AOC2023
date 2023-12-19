pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn hash(str: &str) -> usize {
    let mut curr_val = 0;
    for c in str.chars() {
        let char_val = if c.is_lowercase() {
            c.to_ascii_lowercase()
        } else {
            c.to_ascii_uppercase()
        } as usize; // Technically it's a u8 but this can fit the x17
        curr_val = (curr_val + char_val) * 17 % 256;
    }
    curr_val
}

fn main(input: String, pt1: bool) {
    let init_seq = input.split(',');

    // That was easy, for a day 15 question
    if pt1 {
        let mut sum = 0;
        for op in init_seq {
            sum += hash(op);
        }
        println!("The sum is {sum}");
        return;
    }

    let mut boxes: Vec<Vec<(&str, usize)>> = (0..256).map(|_| Vec::new()).collect();
    for op in init_seq {
        let [key, lens] = op.split(['-', '=']).collect::<Vec<_>>()[..] else {
            panic!("!= 1x - or =");
        };
        let curr_box = &mut boxes[hash(key)];
        if lens != "" {
            let lens_n = lens.parse::<usize>().unwrap();
            if curr_box.iter().any(|(k, _)| *k == key) {
                let idx = curr_box
                    .iter()
                    .enumerate()
                    .find(|(_, (k, _))| *k == key)
                    .unwrap()
                    .0;
                curr_box[idx].1 = lens_n;
            } else {
                curr_box.push((key, lens_n));
            }
        } else if curr_box.iter().any(|(k, _)| *k == key) {
            let idx = curr_box
                .iter()
                .enumerate()
                .find(|(_, (k, _))| *k == key)
                .unwrap()
                .0;
            curr_box.remove(idx);
        }
    }

    // println!(
    //     "{:#?}",
    //     boxes.iter().filter(|b| !b.is_empty()).collect::<Vec<_>>()
    // );
    let mut sum = 0;
    for (b_idx, c_box) in boxes.iter().enumerate() {
        for (idx, (_, lens)) in c_box.iter().enumerate() {
            sum += (b_idx + 1) * (idx + 1) * lens;
        }
    }
    println!("Total lens power: {}", sum);
}

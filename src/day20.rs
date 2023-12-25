use crate::helpers::{first_n, lcm};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

enum ModType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn push_button(
    modules: &HashMap<String, (ModType, Vec<&str>)>,
    ff_mem: &mut HashMap<String, bool>,
    cj_mem: &mut HashMap<String, HashMap<String, bool>>,
    single_op: Option<&str>,
) -> (i32, i32) {
    let (mut p_high, mut p_low) = (0, 0);
    let mut pulses = VecDeque::new();
    pulses.push_back(("broadcaster", "button", false));

    while let Some((target, origin, high)) = pulses.pop_front() {
        if high {
            p_high += 1;
        } else {
            p_low += 1;
        }
        //println!("{origin} -{}-> {target} ({p_high}/{p_low})", if high {"high"} else {"low"});
        if !modules.contains_key(target) {
            continue;
        }
        let (t, outs) = modules.get(target).unwrap();
        if let Some(nhigh) = match t {
            ModType::FlipFlop if !high => {
                let nval = !ff_mem.get(target).unwrap();
                ff_mem.insert(target.to_string(), nval);
                Some(nval)
            }
            ModType::Conjunction => {
                let mut nval = cj_mem.get(target).unwrap().clone();
                nval.insert(origin.to_string(), high);
                cj_mem.insert(target.to_string(), nval.clone());
                Some(nval.iter().any(|(_, was_high)| !was_high))
            }
            ModType::Broadcast => Some(high),
            _ => None,
        } {
            for arm in outs {
                if let Some(opname) = single_op {
                    if target == "broadcaster" && &opname != arm {
                        continue;
                    }
                }
                pulses.push_back((arm, target, nhigh));
            }
        }
    }

    (p_high, p_low)
}

fn mem_hash(
    ff_mem: &HashMap<String, bool>,
    cj_mem: &HashMap<String, HashMap<String, bool>>,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    ff_mem.iter().collect::<Vec<_>>().hash(&mut hasher);
    cj_mem
        .iter()
        .map(|(k, h)| (k, h.iter().collect::<Vec<_>>()))
        .collect::<Vec<_>>()
        .hash(&mut hasher);
    hasher.finish()
}

fn main(input: String, pt1: bool) {
    let modules: HashMap<String, (ModType, Vec<&str>)> = HashMap::from_iter(
        input
            .lines()
            .map(|line| first_n(&mut line.split(" -> ")))
            .map(|[n, outs]| {
                (
                    n.replace(['%', '&'], ""),
                    (
                        match n.chars().next().unwrap() {
                            '%' => ModType::FlipFlop,
                            '&' => ModType::Conjunction,
                            _ => ModType::Broadcast,
                        },
                        outs.split(", ").collect::<Vec<_>>(),
                    ),
                )
            }),
    );
    let mut ff_mem: HashMap<String, bool> = HashMap::new();
    let mut cj_mem: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for (name, (t, _)) in &modules {
        match t {
            ModType::FlipFlop => {
                ff_mem.insert(name.to_string(), false);
            }
            ModType::Conjunction => {
                cj_mem.insert(
                    name.to_string(),
                    HashMap::from_iter(
                        modules
                            .iter()
                            .filter(|(_, (_, c))| c.contains(&name.as_str()))
                            .map(|(n, _)| (n.clone(), false)),
                    ),
                );
            }
            ModType::Broadcast => (),
        }
    }

    if pt1 {
        let (mut high, mut low) = (0, 0);
        let mut iters = 0;
        let init_hash = mem_hash(&ff_mem, &cj_mem);
        while iters != 1_000 {
            println!("iters: {iters}");
            let (n_high, n_low) = push_button(&modules, &mut ff_mem, &mut cj_mem, None);
            iters += 1;
            high += n_high;
            low += n_low;

            let h = mem_hash(&ff_mem, &cj_mem);
            if h == init_hash {
                break;
            }
        }
        println!("{high} high, {low} low in {iters} iterations");
        let loops = 1_000 / iters;
        high *= loops;
        low *= loops;
        iters *= loops;
        while iters != 1_000 {
            //println!("------------------");
            let (n_high, n_low) = push_button(&modules, &mut ff_mem, &mut cj_mem, None);
            iters += 1;
            high += n_high;
            low += n_low;
        }
        println!("{high} high, {low} low in {iters} iterations");
        println!("Result: {}", high * low);
        return;
    }

    let mut time_to_zero = Vec::new();
    for sub_name in &modules.get("broadcaster").unwrap().1 {
        let mut iters = 0;
        let init_cj = cj_mem.clone();
        let init_hash = mem_hash(&ff_mem, &cj_mem);
        loop {
            if iters % 1_000 == 0 {
                println!("iters: {iters}");
            }
            push_button(&modules, &mut ff_mem, &mut cj_mem, Some(sub_name));
            iters += 1;

            let h = mem_hash(&ff_mem, &init_cj);
            if h == init_hash {
                break;
            }
        }
        time_to_zero.push(iters);
    }
    println!("Result: {}", lcm(time_to_zero.as_slice()));
    // Genuinely not a clue why this works
    // I think it has something to do with the circuits connected to the broadcaster
    // being independent up until a final set of conjunctions, but IDK
    // All I know is that adventofcode.com accepts the answer it gives.
}

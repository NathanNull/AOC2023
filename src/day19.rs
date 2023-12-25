use std::collections::{HashMap, VecDeque};
use crate::helpers::first_n;

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn attr(&self, atype: char) -> usize {
        match atype {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid attr type"),
        }
    }

    fn set(&mut self, atype: char, val: usize) {
        match atype {
            'x' => self.x = val,
            'm' => self.m = val,
            'a' => self.a = val,
            's' => self.s = val,
            _ => panic!("Invalid attr type"),
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
enum Op {
    Greater(char, usize),
    Lesser(char, usize),
    Accept,
}

impl Op {
    fn test(&self, part: &Part) -> bool {
        match self {
            Self::Greater(atype, tval) => part.attr(*atype) > *tval,
            Self::Lesser(atype, tval) => part.attr(*atype) < *tval,
            Self::Accept => true,
        }
    }
}

#[derive(Debug)]
struct Rule {
    op: Op,
    output: String,
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    let [sec1, sec2] = first_n(&mut input.split("\r\n\r\n"));

    let workflows: HashMap<&str, _> = HashMap::from_iter(sec1.lines().map(|line| {
        let [name, rules] = first_n(&mut line.split(['{', '}']));
        let r_parsed = rules
            .split(',')
            .map(|rule| match rule.split(':').collect::<Vec<_>>()[..] {
                [test, new] => {
                    let gt = test.contains('>');
                    let [atype, val] = first_n(&mut test.split(['>', '<']));
                    let (a, v) = (atype.chars().next().unwrap(), val.parse::<usize>().unwrap());

                    Rule {
                        op: if gt {
                            Op::Greater(a, v)
                        } else {
                            Op::Lesser(a, v)
                        },
                        output: new.to_string(),
                    }
                }
                [new] => Rule {
                    op: Op::Accept,
                    output: new.to_string(),
                },
                _ => panic!("Invalid rule"),
            })
            .collect::<Vec<_>>();
        (name, r_parsed)
    }));

    if pt1 {
        let parts = sec2
            .lines()
            .map(|line| {
                let [x, m, a, s] =
                    first_n(&mut line.replace(['{', '}'], "").split(',').map(|val| {
                        first_n::<2, _>(&mut val.split('='))[1]
                            .parse::<usize>()
                            .unwrap()
                    }));
                Part { x, m, a, s }
            })
            .collect::<Vec<_>>();

        println!("Finished parsing data");

        let mut completed: Vec<Part> = Vec::new();
        for part in parts {
            let mut curr_flow = "in";
            while !["A", "R"].contains(&curr_flow) {
                let flow = workflows.get(curr_flow).unwrap();
                for rule in flow {
                    if rule.op.test(&part) {
                        curr_flow = &rule.output;
                        break;
                    }
                }
            }
            match curr_flow {
                "A" => completed.push(part),
                "R" => (),
                _ => panic!("Wait what"),
            }
        }

        println!(
            "Completed value: {:?}",
            completed.iter().map(|p| p.value()).sum::<usize>()
        );
        return;
    }

    let mut p_sum = 0;
    let mut queue = VecDeque::new();
    queue.push_back((
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
        "in",
    ));
    while let Some((rs, re, flow_name)) = queue.pop_front() {
        if flow_name == "A" {
            p_sum += i64::abs((rs.x as i64 - 1 - re.x as i64)
                * (rs.m as i64 - 1 - re.m as i64)
                * (rs.a as i64 - 1 - re.a as i64)
                * (rs.s as i64 - 1 - re.s as i64)) as usize;
            continue;
        } else if flow_name == "R" {
            continue;
        }
        let flow = workflows.get(flow_name).unwrap();
        let (mut r_start, mut r_end) = (rs, re);
        for rule in flow {
            if let Some((ns, ne)) = match (rule.op.test(&r_start), rule.op.test(&r_end)) {
                // All pass
                (true, true) => {
                    queue.push_back((r_start.clone(), r_end.clone(), &rule.output));
                    None
                }
                // All fail
                (false, false) => Some((r_start.clone(), r_end.clone())),
                // Some pass, some fail
                (s_in, _) => {
                    let (os, oe, t, val) = match rule.op {
                        Op::Greater(t, val) => (1, 0, t, val),
                        Op::Lesser(t, val) => (0, -1, t, val),
                        _ => panic!("How do you fail an accept"),
                    };
                    let mut new_s = r_start.clone();
                    new_s.set(t, (val as i32 + os) as usize);
                    let mut new_e = r_end.clone();
                    new_e.set(t, (val as i32 + oe) as usize);
                    if s_in {
                        queue.push_back((r_start.clone(), new_e, &rule.output));
                        Some((new_s, r_end.clone()))
                    } else {
                        queue.push_back((new_s, r_end.clone(), &rule.output));
                        Some((r_start.clone(), new_e))
                    }
                }
            } {
                // If values were returned from that match, reset the range
                r_start = ns;
                r_end = ne;
            } else {
                // Otherwise everything must have passed
                break;
            }
        }
    }

    println!("Sum: {p_sum}")
}

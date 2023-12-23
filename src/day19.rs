use std::collections::HashMap;

#[derive(Debug)]
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

fn first_n<const N: usize, T: Default + Copy>(iter: &mut impl Iterator<Item = T>) -> [T; N] {
    let mut ret: [T; N] = [T::default(); N];
    for idx in 0..N {
        ret[idx] = iter.next().unwrap();
    }
    ret
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

    let parts = sec2
        .lines()
        .map(|line| {
            let [x, m, a, s] = first_n(&mut line.replace(['{', '}'], "").split(',').map(|val| {
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
        while !["A","R"].contains(&curr_flow) {
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
            _ => panic!("Wait what")
        }
    }

    println!("Completed value: {:?}", completed.iter().map(|p|p.value()).sum::<usize>());
}

use crate::helpers::first_n;

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}
impl Pos {
    fn from_arr(arr: [usize; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Brick {
    least: Pos,
    most: Pos,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        let rest_z = self.most.z + 1 == other.least.z;
        let over_x = self.least.x <= other.most.x && self.most.x >= other.least.x;
        let over_y = self.least.y <= other.most.y && self.most.y >= other.least.y;
        rest_z && over_x && over_y
    }

    fn drop(&mut self) {
        self.least.z -= 1;
        self.most.z -= 1;
    }
}

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn fall(bricks: Vec<Brick>) -> (usize, Vec<Brick>) {
    let mut fallen: Vec<Brick> = Vec::new();
    let mut n_bricks = bricks;
    n_bricks.sort_by_key(|brick| brick.least.z);
    let mut fell = 0;
    for brick in n_bricks {
        let mut new_brick = brick;
        let mut falling = true;
        if &brick.least.z == &1 {
            falling = false;
        } else {
            for s_brick in &fallen {
                if s_brick.overlaps(&brick) {
                    falling = false;
                    break;
                }
            }
        }
        if falling {
            fell += 1;
            new_brick.drop();
        }
        fallen.push(new_brick);
    }
    (fell, fallen)
}

fn main(input: String, pt1: bool) {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let [bfl, tbr] = first_n::<2, _>(&mut line.split('~').map(|pos| {
                first_n::<3, &str>(&mut pos.split(',')).map(|p| p.parse::<usize>().unwrap())
            }));
            Brick {
                least: Pos::from_arr(bfl),
                most: Pos::from_arr(tbr),
            }
        })
        .collect();

    // Settle until there's no settling to do
    let mut settling = true;
    while settling {
        let (fell, b) = fall(bricks);
        bricks = b;
        settling = fell != 0;
    }
    println!("Done falling");

    if pt1 {
        let mut can_disintegrate = 0;
        for (idx, _) in (&bricks).iter().enumerate() {
            let mut test_set = bricks.clone();
            test_set.remove(idx);
            let (fell, _) = fall(test_set);
            if fell == 0 {
                can_disintegrate += 1;
            }
        }

        println!("Disintegratable bricks: {}", can_disintegrate);
    } else {
        let mut total_fell = 0;
        for (idx, _) in (&bricks).iter().enumerate() {
            let mut test_set = bricks.clone();
            test_set.remove(idx);
            let (fell, _) = fall(test_set);
            total_fell += fell;
        }

        println!("Total bricks fell: {}", total_fell);
    }
}

use std::collections::vec_deque::VecDeque;
use std::cmp::max;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

#[derive(PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
    fn rev(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    fn mirror(&self, ld: bool) -> Self {
        let rev = self.rev();
        match if ld { &rev } else { self } {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
}

#[derive(PartialEq, Clone)]
enum CellType {
    Empty,
    MirrorLU,
    MirrorLD,
    SplitterH,
    SplitterV,
}

#[derive(Clone)]
struct Cell {
    light_dirs: Vec<Dir>,
    cell_type: CellType,
}
impl Cell {
    fn new(ct: CellType) -> Self {
        Self {
            light_dirs: Vec::new(),
            cell_type: ct,
        }
    }
}
impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            if self.light_dirs.len() == 0 || self.cell_type != CellType::Empty {
                match self.cell_type {
                    CellType::Empty => '.',
                    CellType::MirrorLU => '/',
                    CellType::MirrorLD => '\\',
                    CellType::SplitterH => '-',
                    CellType::SplitterV => '|',
                }
            } else {
                self.light_dirs.len().to_string().chars().next().unwrap()
            }
        )
    }
}

fn fire_beam(r_cells: &Vec<Vec<Cell>>, s_pos: (i32, i32, Dir)) -> i32 {
    let mut cells = r_cells.clone();
    let mut beams = VecDeque::new();
    beams.push_back(s_pos);

    while !beams.is_empty() {
        let (x_i, y_i, dir) = beams.pop_front().unwrap();
        if y_i >= cells.len() as i32
            || y_i < 0
            || x_i >= cells[y_i as usize].len() as i32
            || x_i < 0
        {
            //println!("off edge at {:?}", (x_i, y_i));
            continue;
        }
        let (x, y) = (x_i as usize, y_i as usize);
        let curr_cell = &mut cells[y][x];
        if curr_cell.light_dirs.contains(&dir) {
            //println!("looped at {:?}", (x, y));
            continue;
        }
        curr_cell.light_dirs.push(dir.clone());

        // spaghetto
        for beam in match (&curr_cell.cell_type, &dir) {
            // Empty cell
            (CellType::Empty, _) => vec![(x_i + dir.offset().0, y_i + dir.offset().1, dir)],
            // Mirror
            (CellType::MirrorLU | CellType::MirrorLD, _) => {
                let m_dir = dir.mirror(curr_cell.cell_type == CellType::MirrorLD);
                let (ox, oy) = m_dir.offset();
                vec![(x_i + ox, y_i + oy, m_dir)]
            }
            // Splitter (pass through)
            (CellType::SplitterV, Dir::Up | Dir::Down)
            | (CellType::SplitterH, Dir::Left | Dir::Right) => {
                vec![(x_i + dir.offset().0, y_i + dir.offset().1, dir)]
            }
            // Splitter (split)
            (CellType::SplitterV, _) => vec![(x_i, y_i + 1, Dir::Down), (x_i, y_i - 1, Dir::Up)],
            (CellType::SplitterH, _) => vec![(x_i + 1, y_i, Dir::Right), (x_i - 1, y_i, Dir::Left)],
        } {
            beams.push_back(beam);
        }
    }

    cells
        .iter()
        .map(|row| row.iter().map(|c| c.light_dirs.len() > 0))
        .flatten()
        .fold(0, |a, c| a + if c { 1 } else { 0 })
}

fn main(input: String, pt1: bool) {
    let cells: Vec<Vec<Cell>> = input
        .split("\r\n")
        .map(|row| {
            row.chars()
                .map(|c| {
                    Cell::new(match c {
                        '.' => CellType::Empty,
                        '/' => CellType::MirrorLU,
                        '\\' => CellType::MirrorLD,
                        '-' => CellType::SplitterH,
                        '|' => CellType::SplitterV,
                        _ => panic!("Invalid char"),
                    })
                })
                .collect()
        })
        .collect();

    // println!(
    //     "{}",
    //     cells
    //         .iter()
    //         .map(|row| row
    //             .iter()
    //             .map(|c| format!("{:?}", c))
    //             .collect::<Vec<_>>()
    //             .join(""))
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );

    if pt1 {
        let energized = fire_beam(&cells, (0, 0, Dir::Right));
        println!("Number of tiles energized: {energized}");
        return;
    }

    // Startlingly fast for how brute-force-y it is
    let mut max_energized = 0;
    let (max_x, max_y) = (cells[0].len() as i32 - 1, cells.len() as i32 - 1);
    for v_start in 0..(&cells)[0].len() as i32 {
        let top = fire_beam(&cells, (v_start, 0, Dir::Down));
        let bottom = fire_beam(&cells, (v_start, max_y, Dir::Up));
        max_energized = max(top, max(bottom, max_energized));
    }

    println!("Done top/bottom");

    for h_start in 0..(&cells)[0].len() as i32 {
        let left = fire_beam(&cells, (0, h_start, Dir::Right));
        let right = fire_beam(&cells, (max_x, h_start, Dir::Left));
        max_energized = max(left, max(right, max_energized));
    }

    println!("Max energized found is {max_energized}")
}

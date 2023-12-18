extern crate find_folder;
extern crate regex;

use find_folder::Search;
use std::fs::read_to_string;

const DAY: usize = 14;
const PART: usize = 1;
const IS_TEST: bool = false;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    let days = [
        [day1::pt1, day1::pt2],
        [day2::pt1, day2::pt2],
        [day3::pt1, day3::pt2],
        [day4::pt1, day4::pt2],
        [day5::pt1, day5::pt2],
        [day6::pt1, day6::pt2],
        [day7::pt1, day7::pt2],
        [day8::pt1, day8::pt2],
        [day9::pt1, day9::pt2],
        [day10::pt1, day10::pt2],
        [day11::pt1, day11::pt2],
        [day12::pt1, day12::pt2],
        [day13::pt1, day13::pt2],
        [day14::pt1, day14::pt2],
    ];
    let assets = Search::ParentsThenKids(3, 3).for_folder("inputs").unwrap();
    let input_path = assets.join(std::format!(
        "day{}{}.txt",
        DAY,
        if IS_TEST { "-test" } else { "" }
    ));
    let input = read_to_string(input_path).unwrap();
    days[DAY - 1][PART - 1](input);
}

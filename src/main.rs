extern crate find_folder;
extern crate regex;

use find_folder::Search;
use std::fs::read_to_string;

const DAY: usize = 3;
const PART: usize = 2;
const IS_TEST: bool = true;

mod day1;
mod day2;
mod day3;

fn main() {
    let days = [[day1::pt1, day1::pt2], [day2::pt1, day2::pt2], [day3::pt1, day3::pt2]];
    let assets = Search::ParentsThenKids(3, 3).for_folder("inputs").unwrap();
    let input_path = assets.join(std::format!("day{}{}.txt", DAY, if IS_TEST {"-test"} else {""}));
    let input = read_to_string(input_path).unwrap();
    days[DAY-1][PART - 1](input);
}
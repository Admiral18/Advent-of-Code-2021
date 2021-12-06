use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input =
        fs::read_to_string(r#"inputs/level6.txt"#).expect("Something went wrong reading the file");
    let lines = input.split("\n").collect::<Vec<_>>();
    day6::part2(lines);
}

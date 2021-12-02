use std::fs;

mod day1;
mod day2;

fn main() {
    let input =
        fs::read_to_string(r#"inputs\level2.txt"#).expect("Something went wrong reading the file");
    let lines = input.split("\n").collect::<Vec<_>>();
    day2::part2(lines);
}

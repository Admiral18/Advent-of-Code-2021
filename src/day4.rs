#[allow(unused)]
pub(crate) fn part2(lines: Vec<&str>) {
    let mut line_iter = lines.into_iter();
    let drawn_numbers = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|num_str| num_str.parse::<i32>().unwrap());
    // skip the empty line
    line_iter.next();

    let mut boards = parse_boards(&mut line_iter);

    for cur_num in drawn_numbers {
        let mut remaining_boards = vec![];
        let board_count = boards.len();
        for board in &mut boards {
            if board.mark_num(cur_num) {
                if board.check_finished() {
                    if board_count == 1 {
                        // the last board is winning now
                        let result = board.calc_unmarked_sum() * cur_num;
                        println!("Result: {}", result);
                        return;
                    }
                    continue;
                }
            }
            remaining_boards.push(board.clone());
        }
        boards = remaining_boards;
    }
}

#[allow(unused)]
pub(crate) fn part1(lines: Vec<&str>) {
    let mut line_iter = lines.into_iter();
    let drawn_numbers = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|num_str| num_str.parse::<i32>().unwrap());
    // skip the empty line
    line_iter.next();

    let mut boards = parse_boards(&mut line_iter);

    for cur_num in drawn_numbers {
        let debug = 1;
        for board in &mut boards {
            if board.mark_num(cur_num) {
                if board.check_finished() {
                    let result = board.calc_unmarked_sum() * cur_num;
                    println!("Result: {}", result);
                    return;
                }
            }
        }
    }
}

#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<i32>>,
    checked: Vec<Vec<bool>>,
}

impl Board {
    fn new() -> Self {
        Board {
            numbers: Vec::with_capacity(5),
            checked: vec![vec![false; 5]; 5],
        }
    }

    fn calc_unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.checked[y][x] {
                    sum += self.numbers[y][x];
                }
            }
        }
        sum
    }

    fn mark_num(&mut self, num: i32) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                if self.numbers[y][x] == num {
                    self.checked[y][x] = true;
                    return true;
                }
            }
        }
        false
    }

    fn check_finished(&self) -> bool {
        for row in &self.checked {
            if row.iter().all(|val| *val) {
                return true;
            }
        }

        'outer: for x in 0..5 {
            for y in 0..5 {
                if !self.checked[y][x] {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }
}

fn parse_boards<'a>(mut iter: impl Iterator<Item = &'a str>) -> Vec<Board> {
    let mut boards = vec![];
    while let Some(mut line) = iter.next() {
        let mut board = Board::new();

        for _ in 0..5 {
            board.numbers.push(
                line.split_whitespace()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect(),
            );
            line = iter.next().unwrap();
        }
        boards.push(board);
    }
    boards
}

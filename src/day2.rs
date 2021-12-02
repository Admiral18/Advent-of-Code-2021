#[allow(unused)]
pub(crate) fn part2(lines: Vec<&str>) {
    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;
    lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (parts[0], parts[1].parse::<i32>().unwrap())
        })
        .for_each(|(command, value)| match command {
            "forward" => {
                horizontal += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            unexpected => panic!("Error while handling my tuples: {}", unexpected),
        });

    let result = horizontal * depth;
    println!("Result: {}", result);
}

#[allow(unused)]
pub(crate) fn part1(lines: Vec<&str>) {
    let mut horizontal = 0;
    let mut depth = 0;
    lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            (parts[0], parts[1].parse::<i32>().unwrap())
        })
        .for_each(|(command, value)| match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            unexpected => panic!("Error while handling my tuples: {}", unexpected),
        });

    let result = horizontal * depth;
    println!("Result: {}", result);
}

#[allow(unused)]
pub fn part1(lines: Vec<&str>) {
    //let mut last = string_vec[0] + string_vec[1] + string_vec[2];
    let mut last = -1;
    let mut inc_count = 0;

    for cur_val in lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|str_line| str_line.parse::<i32>().unwrap())
    {
        if last != -1 {
            if cur_val > last {
                inc_count += 1;
            }
        }
        last = cur_val;
    }

    println!("Result: {}", inc_count);
}

#[allow(unused)]
pub fn part2(lines: Vec<&str>) {
    let string_vec = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|str_line| str_line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut last = string_vec[0] + string_vec[1] + string_vec[2];
    let mut inc_count = 0;

    for i in 3..string_vec.len() {
        let cur_window_val = last - string_vec[i - 3] + string_vec[i];
        if cur_window_val > last {
            inc_count += 1;
        }
        last = cur_window_val;
    }
    println!("Result: {}", inc_count);
}

#[allow(unused)]
pub(crate) fn part2(lines: Vec<&str>) {
    let non_empty_lines = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    // TODO: how to solve this more elegant without clone?
    let oxygen = filter_to_value(non_empty_lines.clone(), true);
    let co2 = filter_to_value(non_empty_lines, false);
    let result = oxygen * co2;

    println!(
        "Oxygen generator rating: {}, CO2 scrubber rating: {}",
        oxygen, co2
    );
    println!("Result: {}", result);
}

fn filter_to_value(mut numbers: Vec<&str>, most_common: bool) -> usize {
    let mut idx = 0;
    loop {
        numbers = filter_numbers(numbers, idx, most_common);
        if numbers.len() == 1 {
            break;
        }
        idx += 1;
    }
    usize::from_str_radix(numbers[0], 2).unwrap()
}

fn filter_numbers(binary_numbers: Vec<&str>, idx: usize, most_common: bool) -> Vec<&str> {
    let cur_num_count = binary_numbers.len();
    let zero_count = count_zero_occurences_at_index(&binary_numbers, idx);

    let selected_val = if zero_count * 2 > cur_num_count {
        if most_common {
            '0'
        } else {
            '1'
        }
    } else {
        // the case that there are the same number of zeroes and ones
        // falls into this branch and defaults to 1 for oxygen
        // and 0 for CO2, just as specified
        if most_common {
            '1'
        } else {
            '0'
        }
    };

    binary_numbers
        .into_iter()
        .filter(|binary_num| binary_num.chars().nth(idx).unwrap() == selected_val)
        .collect::<Vec<_>>()
}

fn count_zero_occurences_at_index(binary_numbers: &Vec<&str>, idx: usize) -> usize {
    //let binary_num_len = binarynumbers[0].len();
    let mut zero_counter = 0;

    binary_numbers.iter().for_each(|line| {
        if line.chars().nth(idx).unwrap() == '0' {
            zero_counter += 1;
        }
    });
    zero_counter
}

#[allow(unused)]
pub(crate) fn part1(lines: Vec<&str>) {
    let number_count = lines.len();
    let binary_num_len = lines[0].len();
    let binary_num_len_minus_1 = lines[0].len() - 1;

    let mut position_zero_counter = count_occurences(lines);
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..binary_num_len {
        let one_count = number_count - position_zero_counter[i];
        println!(
            "Pos {}: 0-Count: {} 1-Count: {}",
            i, position_zero_counter[i], one_count
        );
        if (position_zero_counter[i] > one_count) {
            gamma_rate |= 0b1 << (binary_num_len_minus_1 - i);
        } else {
            epsilon_rate |= 0b1 << (binary_num_len_minus_1 - i);
        }
    }

    let result = gamma_rate * epsilon_rate;

    println!("Gamma rate: {}, Epsilon rate: {}", gamma_rate, epsilon_rate);
    println!("Result: {}", result);
}

fn count_occurences(binarynumbers: Vec<&str>) -> Vec<usize> {
    let binary_num_len = binarynumbers[0].len();
    let mut position_zero_counter = vec![0; binary_num_len];

    binarynumbers
        .into_iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            for (i, char) in line.chars().enumerate() {
                if char == '0' {
                    position_zero_counter[i] += 1;
                }
            }
        });
    position_zero_counter
}

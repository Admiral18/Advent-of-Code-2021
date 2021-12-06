use std::collections::HashMap;

#[allow(dead_code)]
pub fn part2(lines: Vec<&str>) {
    let input = lines[0];
    let initial_days = input.split(',').map(|day_str| day_str.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut map = HashMap::<i32, i64>::new();
    for i in 0..9 {
        map.insert(i, 0);
    }

    for cur_day in initial_days.iter() {
        *map.get_mut(cur_day).unwrap() += 1;
    }
    
    for _ in 0..256 {
        let old_0 = *map.get(&0).unwrap();
        for i in 1..9{
            let cur_val = map.get(&i);
            if let Some(&val) = cur_val {
                map.insert(i-1, val);
            }
        }
        *map.get_mut(&6).unwrap() += old_0;
        map.insert(8, old_0);
    }

    let result : i64 = map.iter().map(|(_, val)| val).sum();
    println!("Result: {}", result);
}

#[allow(dead_code)]
pub fn part1(lines: Vec<&str>) {
    let input = lines[0];
    let mut initial_days = input.split(',').map(|day_str| day_str.parse::<i32>().unwrap()).collect::<Vec<_>>();
    for _ in 0..80 {
        for i in 0..initial_days.len() {
            let cur_val = initial_days[i];
            if cur_val == 0 {
                initial_days.push(8);
                initial_days[i] = 6;
            } else {
                initial_days[i] = cur_val - 1;
            }
        }
    }
    println!("Result: {}", initial_days.len());
}
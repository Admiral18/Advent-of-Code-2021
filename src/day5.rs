#[allow(dead_code)]
pub(crate) fn part2(lines: Vec<&str>) {
    let mut map = vec![vec![0; 1000]; 1000];

    println!("Start day 5 calc");
    let line_definitions = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line_str| {
            let parts = line_str.split(" -> ").collect::<Vec<_>>();
            let start_pos_coords = parts[0].split(',').collect::<Vec<_>>();
            let end_pos_coords = parts[1].split(',').collect::<Vec<_>>();
            (
                Point::new(
                    start_pos_coords[0].parse().unwrap(),
                    start_pos_coords[1].parse().unwrap(),
                ),
                (Point::new(
                    end_pos_coords[0].parse().unwrap(),
                    end_pos_coords[1].parse().unwrap(),
                )),
            )
        })
        .collect::<Vec<_>>();

    for (start, end) in line_definitions {
        let delta_x = (end.x - start.x) as f64;
        let delta_y = (end.y - start.y) as f64;

        let mut cur_x = start.x as f64;
        let mut cur_y = start.y as f64;
        let mut cur_grid_x = start.x;
        let mut cur_grid_y = start.y;

        let change_x = if delta_x as i32 == 0 {
            0
        } else {
            delta_x as i32 / (delta_x as i32).abs()
        };
        let change_y = if delta_y as i32 == 0 {
            0
        } else {
            delta_y as i32 / (delta_y as i32).abs()
        };

        // mark initial position
        map[cur_grid_x as usize][cur_grid_y as usize] += 1;
        while !(cur_grid_x == end.x && cur_grid_y == end.y) {
            let diff_x = calc_diff(cur_x, delta_x);

            let steps_to_next_x = if delta_x == 0f64 {
                f64::MAX
            } else {
                diff_x / delta_x
            };

            let diff_y = calc_diff(cur_y, delta_y);
            let steps_to_next_y = if delta_y == 0f64 {
                f64::MAX
            } else {
                diff_y / delta_y
            };

            let step_to_perform = if steps_to_next_x == steps_to_next_y {
                cur_grid_x += change_x;
                cur_grid_y += change_y;
                steps_to_next_y
            } else if steps_to_next_x > steps_to_next_y {
                cur_grid_y += change_y;
                steps_to_next_y
            } else {
                cur_grid_x += change_x;
                steps_to_next_x
            };

            cur_x += step_to_perform * delta_x;
            cur_y += step_to_perform * delta_y;

            map[cur_grid_x as usize][cur_grid_y as usize] += 1;
        }
    }

    let result = map.into_iter().flatten().filter(|&val| val >= 2).count();
    println!("Result: {}", result);
}

#[allow(dead_code)]
pub(crate) fn part1(lines: Vec<&str>) {
    let mut map = vec![vec![0; 1000]; 1000];

    println!("Start day 5 calc");
    let line_definitions = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line_str| {
            let parts = line_str.split(" -> ").collect::<Vec<_>>();
            let start_pos_coords = parts[0].split(',').collect::<Vec<_>>();
            let end_pos_coords = parts[1].split(',').collect::<Vec<_>>();
            (
                Point::new(
                    start_pos_coords[0].parse().unwrap(),
                    start_pos_coords[1].parse().unwrap(),
                ),
                (Point::new(
                    end_pos_coords[0].parse().unwrap(),
                    end_pos_coords[1].parse().unwrap(),
                )),
            )
        })
        .filter(|(start, end)| start.x == end.x || start.y == end.y)
        .collect::<Vec<_>>();

    for (start, end) in line_definitions {
        let delta_x = (end.x - start.x) as f64;
        let delta_y = (end.y - start.y) as f64;

        let mut cur_x = start.x as f64;
        let mut cur_y = start.y as f64;
        let mut cur_grid_x = start.x;
        let mut cur_grid_y = start.y;

        let change_x = if delta_x as i32 == 0 {
            0
        } else {
            delta_x as i32 / (delta_x as i32).abs()
        };
        let change_y = if delta_y as i32 == 0 {
            0
        } else {
            delta_y as i32 / (delta_y as i32).abs()
        };

        // mark initial position
        map[cur_grid_x as usize][cur_grid_y as usize] += 1;
        while !(cur_grid_x == end.x && cur_grid_y == end.y) {
            let diff_x = calc_diff(cur_x, delta_x);

            let steps_to_next_x = if delta_x == 0f64 {
                f64::MAX
            } else {
                diff_x / delta_x
            };

            let diff_y = calc_diff(cur_y, delta_y);
            let steps_to_next_y = if delta_y == 0f64 {
                f64::MAX
            } else {
                diff_y / delta_y
            };

            let step_to_perform = if steps_to_next_x > steps_to_next_y {
                cur_grid_y += change_y;
                steps_to_next_y
            } else {
                cur_grid_x += change_x;
                steps_to_next_x
            };

            cur_x += step_to_perform * delta_x;
            cur_y += step_to_perform * delta_y;

            map[cur_grid_x as usize][cur_grid_y as usize] += 1;
        }
    }

    let result = map.into_iter().flatten().filter(|val| *val >= 2).count();
    println!("Result: {}", result);
}

fn calc_diff(cur: f64, delta: f64) -> f64 {
    let next = if delta > 0f64 {
        if cur.ceil() == cur {
            cur + 1f64
        } else {
            cur
        }
    } else {
        if cur.floor() == cur {
            cur - 1f64
        } else {
            cur
        }
    };
    next - cur
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

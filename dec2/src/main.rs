use std::env;
use std::fs::read_to_string;
use std::cmp::min;

fn main() {
    let lines = read_lines(env::args().collect::<Vec<String>>()[1].clone());

    // Part 1
    let mut safe = 0;
    'outer: for line in lines.clone() {
        let mut last_level = -1;
        let mut last_direction = 0;
        for level in line.split_whitespace() {
            let level = level.parse::<i32>().unwrap();
            if last_level == -1 {
                last_level = level;
            } else {
                let diff = level - last_level;
                if diff == 0 || diff.abs() > 3 {
                    continue 'outer;
                }
                let direction = if diff > 0 { 1 } else { -1 };
                let direction_diff:i32 = direction - last_direction;
                if direction_diff.abs() > 1 {
                    continue 'outer;
                }
                last_direction = direction;
                last_level = level;
            }
        }
        safe += 1;
    }
    println!("Part 1: {}", safe);

    // Part 2 recursive
    let mut safe = 0;
    for line in lines.clone() {
        let levels = line.split_whitespace()
            .map(|level: &str| level.parse().unwrap())
            .collect::<Vec<i32>>();
        if bad_levels_recursive(0, &levels, -1, -1) < 2 { // Safe if we go down (start at 1 above first)
            safe += 1;
        } else if bad_levels_recursive(0, &levels, -1, 1) < 2 { // Safe if we go up (start at 1 below first)
            safe += 1;
        }
    }
    println!("Part 2 recursive: {}", safe);

}


fn bad_levels_recursive(bad_so_far: i32, levels: &Vec<i32>, last_level: i32, last_direction: i32) -> i32 {
    if bad_so_far >= 2 { // Do not continue if we already have two bad levels
        return bad_so_far;
    }
    return match levels[..] {
        [] => 0,
        _ => {
            let level = levels[0];
            let diff = level - last_level;
            let direction = if diff > 0 { 1 } else { -1 };

            if last_level != -1 && (diff == 0 || diff.abs() > 3 || direction != last_direction) {
                // Skip this level
                bad_levels_recursive(bad_so_far+1, &levels[1..].to_vec(), last_level, last_direction)
            } else {
                min(
                    bad_levels_recursive(bad_so_far + 1, &levels[1..].to_vec(), last_level, last_direction), // Skip this level
                    bad_levels_recursive(bad_so_far, &levels[1..].to_vec(), level, last_direction) // Use this level
                )
            }
        }
    }
}

fn bad_levels(levels: &Vec<i32>) -> i32 {
    let mut last_level = -1;
    let mut last_direction = 0;
    let mut skipped_levels = 0;
    for level in levels {
        if last_level == -1 {
            last_level = *level;
        } else {
            let diff = level - last_level;
            if diff == 0 || diff.abs() > 3 {
                skipped_levels += 1;
                continue;
            }
            let direction = if diff > 0 { 1 } else { -1 };
            let direction_diff: i32 = direction - last_direction;
            if direction_diff.abs() > 1 {
                skipped_levels += 1;
                continue;
            }
            last_direction = direction;
            last_level = *level;
        }
    }
    skipped_levels
}

fn read_lines(filename: String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

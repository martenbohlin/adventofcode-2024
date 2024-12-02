use std::env;
use std::fs::read_to_string;

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

    // Part 2
    let mut safe = 0;
    for line in lines.clone() {
        let levels = line.split_whitespace()
            .map(|level:&str| level.parse().unwrap())
            .collect::<Vec<i32>>();
        let skipped_levels = bad_levels(&levels); // Handles failed once it got started
        if skipped_levels < 2 {
            safe += 1;
        } else {
            if bad_levels(&levels[1..].to_vec()) == 0  { // Was the first one bad?
                safe += 1;
            } else {
                let mut a = levels[0..1].to_vec();
                let b = levels[2..].to_vec();
                a.extend(b);
                if bad_levels(&a) == 0 { // Was the second one bad?
                    safe += 1;
                }
            }
        }
    }
    println!("Part 2: {}", safe);

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

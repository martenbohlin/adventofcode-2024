use std::env;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines(env::args().collect::<Vec<String>>()[1].clone());

    // Part 1
    let mut safe = 0;
    'outer: for line in lines {
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
}

fn read_lines(filename: String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

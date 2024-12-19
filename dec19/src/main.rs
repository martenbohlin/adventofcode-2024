use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let (towels, pattenrs) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&towels, &pattenrs);
    part2(&towels, &pattenrs);
}

fn part1(towels: &Vec<String>, patterns: &Vec<String>) {
    let mut unmatched:HashMap<String, i64> = HashMap::new();
    let mut count = 0;
    for pattern in patterns {
        if can_make_pattern(pattern, towels, &mut unmatched) > 0{
            count += 1;
        }
    }
    println!("Part 1: {:?}", count);
}

fn part2(towels: &Vec<String>, patterns: &Vec<String>) {
    let mut unmatched:HashMap<String, i64> = HashMap::new();
    let mut count = 0;
    for pattern in patterns {
        count += can_make_pattern(pattern, towels, &mut unmatched);
    }
    println!("Part 2: {:?}", count);
}

fn can_make_pattern(pattern: &String, towels: &Vec<String>, calculated: &mut HashMap<String, i64>) -> i64 {
    if pattern.len() == 0 {
        return 1;
    }
    if calculated.contains_key(pattern) {
        return *calculated.get(pattern).unwrap();
    }
    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            count += can_make_pattern(&pattern[towel.len()..].to_string(), towels, calculated);
        }
    }
    calculated.insert(pattern.clone(), count);
    return count;
}

fn read_file(filename: String) -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let towels = lines[0].split(',').map(str::trim).map(String::from).collect::<Vec<String>>();
    return (towels, lines[2..].to_vec());
}

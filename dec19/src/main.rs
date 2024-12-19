use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let (towels, pattenrs) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(towels, pattenrs);
}

fn part1(towels: Vec<String>, patterns: Vec<String>) {
    let mut unmatched:HashSet<String> = HashSet::new();
    let mut count = 0;
    for pattern in patterns {
        if can_make_pattern(&pattern, &towels, &mut unmatched) {
            count += 1;
        }
    }
    println!("Part 1: {:?}", count);
}

fn can_make_pattern(pattern: &String, towels: &Vec<String>, unmatched: &mut HashSet<String>) -> bool {
    if pattern.len() == 0 {
        return true;
    }
    if unmatched.contains(pattern) {
        return false;
    }
    for towel in towels {
        if pattern.starts_with(towel) {
            if can_make_pattern(&pattern[towel.len()..].to_string(), towels, unmatched) {
                return true;
            }
        }
    }
    unmatched.insert(pattern.clone());
    return false;
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

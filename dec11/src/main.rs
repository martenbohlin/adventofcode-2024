use std::collections::{HashMap};
use std::{env};
use std::fs::read_to_string;

fn main() {
    let stones = read_file(env::args().collect::<Vec<String>>()[1].clone());
    println!("{:?}", stones);
    part1(stones.clone());
    part2(stones.clone());
}

fn part1(stones_in: HashMap<i64,i64>) {
    println!("Part 1: {}", iterate_rules(stones_in, 25));
}
fn part2(stones_in: HashMap<i64,i64>) {
    println!("Part 2: {}", iterate_rules(stones_in, 75));
}

fn iterate_rules(stones_in: HashMap<i64, i64>, itterations: i64) -> i64 {
    let mut stones = stones_in.clone();
    for _blink in 0..itterations {
        let mut next_generation = HashMap::new();
        for stones_with_same_number in stones {
            let _ = rule1(&mut next_generation, stones_with_same_number) ||
                rule2(&mut next_generation, stones_with_same_number) ||
                rule3(&mut next_generation, stones_with_same_number);
        }
        stones = next_generation;
    }

    let mut sum = 0;
    for stones_with_same_number in stones {
        sum += stones_with_same_number.1;
    }
    sum
}

fn rule1(stones:&mut HashMap<i64,i64>, stones_with_same_number: (i64, i64)) -> bool {
    match stones_with_same_number {
        (0, count) => {
            add_stones(stones, 1, count);
            true
        },
        _ => false,
    }
}

fn rule2(stones:&mut HashMap<i64,i64>, stones_with_same_number: (i64, i64)) -> bool {
    let (number, count) = stones_with_same_number;
    let digits = if number==0 { 1 }else { number.ilog10() + 1};
    if digits % 2 == 0 {
        let half = digits / 2;
        let exp = 10_i64.pow(half);
        add_stones(stones, number / exp, count);
        add_stones(stones, number % exp, count);
        return true;
    } else {
        return false;
    }
}

fn rule3(stones:&mut HashMap<i64,i64>, stones_with_same_number: (i64, i64)) -> bool {
    let (number, count) = stones_with_same_number;
    add_stones(stones, number * 2024, count);
    return true;
}

fn read_file(filename: String) -> HashMap<i64,i64> {
    let mut result = HashMap::new();
    let vec: Vec<i64> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .split_whitespace()  // split the string into an iterator of string slices
        .map(|number| number.parse::<i64>().unwrap())  // make each slice into a string
        .collect();  // gather them together into a vector
    for number in vec {
        add_stones(&mut result, number, 1);
    }
    return result;
}

fn add_stones(result: &mut HashMap<i64, i64>, number: i64, count: i64) {
    let x = &result.entry(number).or_insert(0).clone();
    result.insert(number, x + count);
}

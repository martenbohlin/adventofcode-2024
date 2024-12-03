use std::env;
use std::fs::read_to_string;
use regex::{Captures, Regex};

fn main() {
    let file = read_to_string(env::args().collect::<Vec<String>>()[1].clone()).unwrap();

    part1(&file);
    part2(&file);
}

fn part1(file: &String) {
    let find_mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Cannot create regex");

    let caps: Vec<Captures<>> = find_mul.captures_iter(&file).collect();
    let mut sum:i64 = 0;
    for mul in caps {
        sum += multiply(&mul);
    }
    println!("Part 1: {}", sum);
}

fn part2(file: &String) {
    let find_expressions = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").expect("Cannot create regex");

    let caps: Vec<Captures<>> = find_expressions.captures_iter(&file).collect();
    let mut sum:i64 = 0;
    let mut active = true;
    for expr in caps {
        match expr.get(0).unwrap().as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => if active { sum += multiply(&expr) },
        }
    }
    println!("Part 1: {}", sum);
}

fn multiply(mul: &Captures) -> i64 {
    let a = num_argument(&mul, 1);
    let b = num_argument(&mul, 2);
    a * b
}

fn num_argument(mul: &Captures, i: usize) -> i64 {
    mul.get(i).unwrap().as_str().parse::<i64>().unwrap()
}

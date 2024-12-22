use std::env;
use std::fs::read_to_string;

fn main() {
    let start = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&start);
}

fn part1(start: &Vec<i64>) {
    let mut sum = 0_i64;
    for x in start {
        let mut y = *x;
        for _i in 0..2000 {
            y = next(&y);
        }
        //println!("{}: {}", x, y);
        sum += y as i64;
    }
    println!("Part 1: {}", sum);
}

fn next(x: &i64) -> i64 {
    let prune = 16777216;
    let mut result = ((x*64) ^x) % prune;
    result = ((result/32) ^ result) % prune;
    result = ((result*2048) ^ result) % prune;
    return result;
}


fn read_file(filename: String) -> Vec<i64> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.parse::<i64>().unwrap())  // make each slice into a string
        .collect()  // gather them together into a vector
}

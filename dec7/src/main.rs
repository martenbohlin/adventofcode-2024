use std::env;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string(env::args().collect::<Vec<String>>()[1].clone())
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect();

    part1(&file);
}

fn part1(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let(result, numbers) = parse_line(line);
        if test(result, numbers[0], &numbers[1..].to_vec()) {
            sum += result;
        }
    }

    println!("Part 1: {}", sum);
}

fn test(result: i64, now: i64, numbers: &Vec<i64>) -> bool {
    return match numbers[..] {
        [] => now == result, // Did we get the correct result?
        _ => {
            test(result, now + numbers[0], &numbers[1..].to_vec()) || // Will + give the correct result?
                test(result, now * numbers[0], &numbers[1..].to_vec()) // Will * give correct result?
        }
    };
}

fn parse_line(line: &String) -> (i64, Vec<i64>) {
    let mut numbers = Vec::new();
    let mut x = line.split(":");
    let result = x.next().unwrap().parse::<i64>().unwrap();

    for number in x.next().unwrap().split_whitespace() {
        numbers.push(number.parse::<i64>().unwrap());
    }
    return (result, numbers);
}

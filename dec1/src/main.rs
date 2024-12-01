use std::fs::read_to_string;
use std::env;

fn main() {
    let lines = read_lines(env::args().collect::<Vec<String>>()[1].clone());
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in lines {
        let numbers = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        a.push(numbers[0]);
        b.push(numbers[1]);
    }

    a.sort();
    b.sort();

    let mut sum = 0;
    for (i, an) in a.iter().enumerate() {
        let bn = b[i];
        sum += (an - bn).abs();
        println!("{} {} {}", an, bn, sum);
    }
    println!("{}", sum);
}

fn read_lines(filename: String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

use std::env;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let machines = parse_file(env::args().collect::<Vec<String>>()[1].clone());

    part1(&machines);
    part2(&machines)
}

fn part1(machines: &Vec<Machine>) {
    let mut sum = 0;
    for machine in machines {
        match solve_part1(machine) {
            Some(tokens) => sum += tokens,
            None => (),
        }
    }
    println!("Part 1: {}", sum);
}

fn solve_part1(machine: &Machine) -> Option<i64> {
    for i in 0..100 {
        if (machine.prize.0 - machine.button_a.0 * i) % machine.button_b.0 == 0 {
            let j = (machine.prize.0 - machine.button_a.0 * i) / machine.button_b.0;
            if machine.prize.1 == machine.button_a.1 * i + machine.button_b.1 * j {
                return Some(i*3+j);
            }
        }
    }
    return None;
}

fn part2(machines: &Vec<Machine>) {
    let mut sum = 0;
    for machine in machines {
        match solve_part2(machine) {
            Some(tokens) => sum += tokens,
            None => (),
        }
    }
    println!("Part 2: {}", sum);
}

fn solve_part2(machine: &Machine) -> Option<i64> {
    let x = machine.prize.0 + 10000000000000;
    let y = machine.prize.1 + 10000000000000;
    let ax = machine.button_a.0;
    let bx = machine.button_b.0;
    let ay = machine.button_a.1;
    let by = machine.button_b.1;

    let a = (by*x - bx*y)/(by*ax - bx*ay);
    if (x - ax * a) % bx == 0 {
        let b = (x - ax * a) / bx;
        if y == ay * a + by * b {
            return Some(a*3 + b);
        }
    }

    return None;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_file(filename: String) -> Vec<Machine> {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let button_a = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").expect("Cannot create regex");
    let button_b = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").expect("Cannot create regex");
    let prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").expect("Cannot create regex");

    let mut result = Vec::new();
    for i in (0..lines.len()).step_by(4) {
        let button_a_captures = button_a.captures(&lines[i]).unwrap();
        let button_a_x = button_a_captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let button_a_y = button_a_captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let button_b_captures = button_b.captures(&lines[i+1]).unwrap();
        let button_b_x = button_b_captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let button_b_y = button_b_captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        let prize_captures = prize.captures(&lines[i+2]).unwrap();
        let prize_x = prize_captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let prize_y = prize_captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

        result.push(Machine {
            button_a: (button_a_x, button_a_y),
            button_b: (button_b_x, button_b_y),
            prize: (prize_x, prize_y),
        });
    }
    return result;
}

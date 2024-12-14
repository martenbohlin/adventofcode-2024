use std::env;
use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let robots = parse_file(env::args().collect::<Vec<String>>()[1].clone());
    let width = env::args().collect::<Vec<String>>()[2].parse::<i64>().unwrap();
    let height = env::args().collect::<Vec<String>>()[3].parse::<i64>().unwrap();

    part1(&robots, width, height, 100);
    part2(&robots, width, height);
}

fn part1(robots: &Vec<((i64,i64),(i64,i64))>, width: i64, height: i64, seconds: i64) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots {
        let ((mut x, mut y), (vx,vy)) = robot;
        x += vx * seconds;
        y += vy * seconds;
        x = x.rem_euclid(width);
        y = y.rem_euclid(height);
        if x < width/2 && y < height/2 {
            q1 += 1;
        } else if x > width/2 && y < height/2 {
            q2 += 1;
        } else if x < width/2 && y > height/2 {
            q3 += 1;
        } else if x > width/2 && y > height/2 {
            q4 += 1;
        }
    }
    println!("Part 1: {} {} {} {} => {}", q1, q2, q3, q4, q1*q2*q3*q4);
}

fn part2(robots: &Vec<((i64,i64),(i64,i64))>, width: i64, height: i64) {
    for seconds in 0..400000 {
        if seconds % 10000 == 0 {
            println!("Second: {}", seconds);
        }
        let mut grid = vec![vec![' '; width as usize]; height as usize];
        let mut robot_pos: Vec<(i64,i64)> = Vec::new();
        for robot in robots {
            let ((mut x, mut y), (vx, vy)) = robot;
            x += vx * seconds;
            y += vy * seconds;
            x = x.rem_euclid(width);
            y = y.rem_euclid(height);
            grid[y as usize][x as usize] = '#';
            robot_pos.push((x,y));
        }
        if !has_overlapping(&robot_pos) {
            println!("Second: {}", seconds);
            for row in grid {
                println!("{}", row.iter().collect::<String>());
            }
            return;
        }
    }
}

fn has_overlapping(robots: &Vec<(i64,i64)>) -> bool {
    for i in 0..robots.len() {
        for j in i+1..robots.len() {
            if robots[i] == robots[j] {
                return true;
            }
        }
    }
    return false;
}

fn parse_file(filename: String) -> Vec<((i64,i64),(i64,i64))> {
    let lines: Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let robot = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").expect("Cannot create regex");

    let mut result = Vec::new();
    for line in lines {
        let robot_captures = robot.captures(&line).unwrap();
        let x = robot_captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = robot_captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vx = robot_captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vy = robot_captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
        result.push(((x,y),(vx,vy)));
    }
    return result;
}

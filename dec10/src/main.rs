use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let grid = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            sum += path_part1(&grid, (x as i32, y as i32), '0', &mut HashSet::new());
        }
    }
    println!("Part 1: {}", sum);
}

fn path_part1(grid: &Vec<Vec<char>>, pos: (i32, i32), expexted_height: char, visited: &mut HashSet<(i32, i32)>) -> usize {
    if !in_grid(&grid, pos) {
        return 0;
    }
    let height = grid[pos.1 as usize][pos.0 as usize];
    if visited.contains(&pos) || height != expexted_height {
        return 0;
    }
    visited.insert(pos);
    if height == '9' {
        return 1;
    }
    let mut sum = 0;
    let next_height = (expexted_height as u8 + 1) as char;
    sum += path_part1(&grid, (pos.0, pos.1-1), next_height, visited);
    sum += path_part1(&grid, (pos.0, pos.1+1), next_height, visited);
    sum += path_part1(&grid, (pos.0-1, pos.1), next_height, visited);
    sum += path_part1(&grid, (pos.0+1, pos.1), next_height, visited);

    return sum;
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            sum += crate::path_part2(&grid, (x as i32, y as i32), '0');
        }
    }
    println!("Part 2: {}", sum);
}

fn path_part2(grid: &Vec<Vec<char>>, pos: (i32, i32), expexted_height: char) -> usize {
    if !in_grid(&grid, pos) {
        return 0;
    }
    let height = grid[pos.1 as usize][pos.0 as usize];
    if height != expexted_height {
        return 0;
    }
    if height == '9' {
        return 1;
    }
    let mut sum = 0;
    let next_height = (expexted_height as u8 + 1) as char;
    sum += crate::path_part2(&grid, (pos.0, pos.1-1), next_height);
    sum += crate::path_part2(&grid, (pos.0, pos.1+1), next_height);
    sum += crate::path_part2(&grid, (pos.0-1, pos.1), next_height);
    sum += crate::path_part2(&grid, (pos.0+1, pos.1), next_height);

    return sum;
}

fn in_grid(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < grid[0].len() as i32 && pos.1 >= 0 && pos.1 < grid.len() as i32
}

fn debug(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}


fn read_file(filename: String) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect()  // gather them together into a vector
}

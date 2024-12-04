use std::env;
use std::fs::read_to_string;

fn main() {
    let grid = read_file(env::args().collect::<Vec<String>>()[1].clone());
    debug(&grid);

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Vec<Vec<char>>) {
    let word = "XMAS".chars().collect();
    let mut words = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            words += search(&grid, &word, x as i32, y as i32);
        }
    }
    println!("Part 1: {}", words);
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut words = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid[y].len()-1 {
            words += mas_in_shape_of_x(&grid, x, y);
        }
    }
    println!("Part 2: {}", words);
}

fn mas_in_shape_of_x(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    if grid[y][x] == 'A' &&
        (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S' || grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M') &&
        (grid[y-1][x+1] == 'M' && grid[y+1][x-1] == 'S' || grid[y-1][x+1] == 'S' && grid[y+1][x-1] == 'M')
    {
        return 1;
    } else {
        return 0;
    }
}

fn search(grid: &Vec<Vec<char>>, word: &Vec<char>, x: i32, y: i32) -> i32 {
    if grid[y as usize][x as usize] != word[0] {
        return 0;
    }
    let mut words = 0;

    let nil = |_i| 0;
    let east = |i| i as i32;
    let west = |i| -(i as i32);
    let south = |i| i as i32;
    let north = |i| -(i as i32);

    words += matching(&grid, &word, x, y, &west, &nil); // West
    words += matching(&grid, &word, x, y, &east, &nil); // East
    words += matching(&grid, &word, x, y, &east, &south); // South-East
    words += matching(&grid, &word, x, y, &west, &south); // South-West
    words += matching(&grid, &word, x, y, &east, &north); // North-East
    words += matching(&grid, &word, x, y, &west, &north); // North-West
    words += matching(&grid, &word, x, y, &nil, &south); // South
    words += matching(&grid, &word, x, y, &nil, &north); // North

    return words;
}

fn matching(grid: &Vec<Vec<char>>, word: &Vec<char>, x: i32, y: i32,
            x_modifier: &dyn Fn(usize) -> i32, y_modifier: &dyn Fn(usize) -> i32) -> i32 {
    for i in 0..word.len() {
        let x2 = x + x_modifier(i);
        let y2 = y + y_modifier(i);
        if x2 < 0 || x2 >= grid[y as usize].len() as i32 ||
            y2 < 0 || y2 >= grid.len() as i32 ||
            grid[y2 as usize][x2 as usize] != word[i] {
            return 0;
        }
    }
    return 1;
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

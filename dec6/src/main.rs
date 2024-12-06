use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let grid = read_file(env::args().collect::<Vec<String>>()[1].clone());

    part1(&grid);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut visited_positions:HashSet<(i32, i32)> = HashSet::new();
    let mut coordinate = find_start_position(&grid);
    let mut direction = Direction::North;
    while in_grid(&grid, coordinate) {
        visited_positions.insert(coordinate);
        (direction, coordinate) = move_guard(&grid, direction, coordinate);
    }

    println!("Part 1: {}", visited_positions.len());
}

fn move_guard(grid: &Vec<Vec<char>>, direction: Direction, coordinate: (i32, i32)) -> (Direction, (i32, i32)) {
    let (x, y) = coordinate;
    let new_coordinate = match direction {
        Direction::North => (x, y - 1),
        Direction::East => (x + 1, y),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
    };
    if in_grid(grid, new_coordinate) && grid[new_coordinate.1 as usize][new_coordinate.0 as usize] == '#'{
        return move_guard(&grid, turn_right(direction), coordinate);
    } else {
        return (direction, new_coordinate);
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => { Direction::East}
        Direction::East => {Direction::South}
        Direction::South => {Direction::West}
        Direction::West => {Direction::North}
    }
}

fn in_grid(grid: &Vec<Vec<char>>, coordinate: (i32, i32)) -> bool {
    let (x, y) = coordinate;
    x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[y as usize].len() as i32
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn find_start_position(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start position found");
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

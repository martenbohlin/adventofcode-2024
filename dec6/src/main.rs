use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let grid = read_file(env::args().collect::<Vec<String>>()[1].clone());

    part1(&grid);
    part2(grid);
}

fn part1(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut visited_positions:HashSet<(i32, i32)> = HashSet::new();
    let mut coordinate = find_start_position(&grid);
    let mut direction = Direction::North;
    while in_grid(&grid, coordinate) {
        visited_positions.insert(coordinate);
        (direction, coordinate) = move_guard(&grid, direction, coordinate);
    }

    println!("Part 1: {}", visited_positions.len());
    return visited_positions;
}

fn part2(mut grid: Vec<Vec<char>>) {
    let start = find_start_position(&grid);
    let mut causing_loop = 0;
    let mut tested = HashSet::new();

    let (path, known) = find_path(&grid, start);

    let mut path_index = 0;
    for (direction, coordinate) in path {
        path_index = path_index + 1;
        if !tested.contains(&coordinate) {
            tested.insert(coordinate);
            grid[coordinate.1 as usize][coordinate.0 as usize] = '#';
            if stuck_in_loop(&grid, coordinate, direction, &known)  {
                causing_loop += 1;
            }
            grid[coordinate.1 as usize][coordinate.0 as usize] = '.';
        }
    }

    println!("Part 2: {}", causing_loop);
}

fn find_path(grid: &Vec<Vec<char>>, start: (i32, i32)) -> (Vec<(Direction,(i32, i32))>,HashSet<(Direction,(i32, i32))>) {
    let mut coordinate = start.clone();
    let mut path:Vec<(Direction,(i32, i32))> = Vec::new();
    let visited_positions:HashSet<(Direction,(i32, i32))> = HashSet::new();
    let mut direction = Direction::North;
    while in_grid(&grid, coordinate) {
        path.push((direction, coordinate));
        (direction, coordinate) = move_guard(&grid, direction, coordinate);
    }
    //println!("Not stuck in loop {:?}", visited_positions);
    return (path, visited_positions);
}

fn stuck_in_loop(grid: &Vec<Vec<char>>, start: (i32, i32), start_direction: Direction, known: &HashSet<(Direction,(i32, i32))>) -> bool {
    let mut coordinate = start.clone();
    let mut visited_positions = known.clone();
    let mut direction = start_direction;
    while in_grid(&grid, coordinate) {
        if visited_positions.contains(&(direction, coordinate)) {
            return true;
        }
        visited_positions.insert((direction, coordinate));
        (direction, coordinate) = move_guard(&grid, direction, coordinate);
    }
    //println!("Not stuck in loop {:?}", visited_positions);
    return false;
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

use std::env;
use std::fs::read_to_string;

fn main() {
    let (map, commands) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let coordinate = find_robot(&map);
    part1(coordinate, &map, commands);
}

fn part1(_coordinate: (usize, usize), _map: &Vec<Vec<char>>, commands: Vec<char>) {
    let mut map = _map.clone();
    let mut coordinate = _coordinate;
    debug_print(&map);
    println!("{:?} {:?}", coordinate, commands);

    for command in commands {
        coordinate = move_robot(&mut map, coordinate, command);
        debug_print(&map);
        println!("{:?} {:?}", coordinate, command);
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn move_robot(map: &mut Vec<Vec<char>>, coordinate: (usize, usize), command: char) -> (usize, usize) {
    let new_coordinate = move_coordinate(&coordinate, &command);
    if !move_box(map, new_coordinate, command) {
        return coordinate;
    }

    map[coordinate.1][coordinate.0] = '.';
    map[new_coordinate.1][new_coordinate.0] = '@';
    return new_coordinate;
}

fn move_box(map: &mut Vec<Vec<char>>, coordinate: (usize, usize), command: char) -> bool {
    return match map[coordinate.1][coordinate.0] {
        '#' => false,
        '.' => true,
        'O' => {
            let new_coordinate = move_coordinate(&coordinate, &command);
            if move_box(map, new_coordinate, command) {
                map[coordinate.1][coordinate.0] = '.';
                map[new_coordinate.1][new_coordinate.0] = 'O';
                true
            } else {
                false
            }
        },
        _ => panic!("Unknown box: {} {:?}", map[coordinate.1][coordinate.0] , coordinate),
    };
}

fn move_coordinate(coordinate: &(usize, usize), command: &char) -> (usize, usize) {
    return match command {
        '^' => (coordinate.0, coordinate.1 - 1),
        '>' => (coordinate.0 + 1, coordinate.1),
        'v' => (coordinate.0, coordinate.1 + 1),
        '<' => (coordinate.0 - 1, coordinate.1),
        _ => {
            panic!("Unknown command: {}", command);
        },
    }
}

fn debug_print(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}

fn find_robot(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '@' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

fn read_file(filename: String) -> (Vec<Vec<char>>, Vec<char>) {
    let lines:Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut map:Vec<Vec<char>> = Vec::new();
    let mut commands:Vec<char> = Vec::new();
    let mut mapping = true;
    for line in lines {
        if line == "" {
            mapping = false;
            continue;
        }
        if mapping {
            let l: Vec<char> = line.chars().collect();
            map.push(l);
        } else {
            for command in line.chars() {
                commands.push(command);
            }
        }
    }
    return (map, commands);
}

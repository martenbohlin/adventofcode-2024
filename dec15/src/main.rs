use std::env;
use std::fs::read_to_string;

fn main() {
    let (map, commands) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let coordinate = find_robot(&map);
    part1(coordinate, &map, &commands);
    part2(coordinate, &map, &commands);
}

fn part1(_coordinate: (usize, usize), _map: &Vec<Vec<char>>, commands: &Vec<char>) {
    let mut map = _map.clone();
    let mut coordinate = _coordinate;
    //debug_print(&map);
    //println!("{:?} {:?}", coordinate, commands);

    for command in commands {
        coordinate = move_robot(&mut map, coordinate, command);
        //debug_print(&map);
        //println!("{:?} {:?}", coordinate, command);
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

fn part2(_coordinate: (usize, usize), _map: &Vec<Vec<char>>, commands: &Vec<char>) {
    let mut map = widen_map(_map);//_map.clone(); //
    let mut coordinate = (_coordinate.0*2, _coordinate.1); //_coordinate.clone();//

    for command in commands {
        coordinate = move_robot(&mut map, coordinate, command);
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '[' {
                sum += 100 * y + x;
            }
        }
    }
    println!("Part 2: {}", sum);
}

fn widen_map(_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for y in 0.._map.len() {
        let mut row = Vec::new();
        for x in 0.._map[y].len() {
            if _map[y][x] == 'O' {
                row.push('[');
                row.push(']');
            } else if _map[y][x] == '@' {
                row.push('@');
                row.push('.');
            } else {
                row.push(_map[y][x]);
                row.push(_map[y][x]);
            }
        }
        map.push(row);
    }
    return map;
}

fn move_robot(map: &mut Vec<Vec<char>>, coordinate: (usize, usize), command: &char) -> (usize, usize) {
    let new_coordinate = move_coordinate(&coordinate, &command);
    if !move_box(map, new_coordinate, command, true) {
        return coordinate;
    }
    move_box(map, new_coordinate, command, false);

    map[coordinate.1][coordinate.0] = '.';
    map[new_coordinate.1][new_coordinate.0] = '@';
    return new_coordinate;
}

fn move_box(map: &mut Vec<Vec<char>>, coordinate: (usize, usize), command: &char, dry_run: bool) -> bool {
    return match map[coordinate.1][coordinate.0] {
        '#' => false,
        '.' => true,
        'O' => {
            let new_coordinate = move_coordinate(&coordinate, &command);
            if move_box(map, new_coordinate, command, dry_run) {
                if !dry_run {
                    map[coordinate.1][coordinate.0] = '.';
                    map[new_coordinate.1][new_coordinate.0] = 'O';
                }
                true
            } else {
                false
            }
        },
        '[' => move_large_box(map, command, coordinate, dry_run),
        ']' => move_large_box(map, command, (coordinate.0-1, coordinate.1), dry_run),
        _ => panic!("Unknown box: {} {:?}", map[coordinate.1][coordinate.0] , coordinate),
    };
}

fn move_large_box(map: &mut Vec<Vec<char>>, command: &char, coordinate:(usize,usize), dry_run: bool) -> bool {
    let new_coordinate = move_coordinate(&coordinate, &command);
    let (x1,y) = coordinate;
    let x2 = x1 + 1;
    let nx1 = new_coordinate.0;
    let nx2 = nx1 + 1;
    let ny = new_coordinate.1;

    map[y][x1] = '.';
    map[y][x2] = '.';
    if move_box(map, (nx1, ny), command, dry_run) && move_box(map, (nx2, ny), command, dry_run) {
        if dry_run {
            map[y][x1] = '[';
            map[y][x2] = ']';
        } else {
            map[ny][nx1] = '[';
            map[ny][nx2] = ']';
        }
        true
    } else {
        map[y][x1] = '[';
        map[y][x2] = ']';
        false
    }
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

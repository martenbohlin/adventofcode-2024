use std::env;
use std::fs::read_to_string;

fn main() {
    let map = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&map);
}

fn part1(map: &Vec<Vec<char>>) {
    let mut distance: Vec<Vec<i32>> = vec![vec![i32::MIN; map[0].len()]; map.len()];
    debug(map);
    let start = find('S', &map);
    run(&start, &mut map.clone(), &mut distance, 0);
    debug(&map);
    let mut count = 0;
    for y in 1..map.len()-1 {
        for x in 1..map[y].len()-1 {
            if map[y][x] == '#' {
                if distance[y-1][x] != i32::MIN && distance[y+1][x] != i32::MIN {
                    let saved = (distance[y - 1][x] - distance[y + 1][x]).abs() - 2;
                    if saved >= 100 {
                        count += 1;
                    }
                }
                if distance[y][x-1] != i32::MIN && distance[y][x+1] != i32::MIN {
                    let saved = (distance[y][x-1] - distance[y][x+1]).abs() - 2;
                    if saved >= 100 {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {:?}", count);
}

fn run(pos: &(usize, usize), map: &mut Vec<Vec<char>>, distance: &mut Vec<Vec<i32>>, dist: i32) {
    let x = pos.0;
    let y = pos.1;
    if map[y][x] == '#' {
        return;
    }
    distance[y][x] = dist;
    if map[y][x] == 'E' {
        return;
    }
    map[y][x] = '#';
    run(&(x, y-1), map, distance, dist+1);
    run(&(x, y+1), map, distance, dist+1);
    run(&(x-1, y), map, distance, dist+1);
    run(&(x+1, y), map, distance, dist+1);
}


fn find(target: char, map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == target {
                return (x, y);
            }
        }
    }
    return (0, 0);
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

use std::collections::{HashMap};
use std::env;
use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let map = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let mut distance: Vec<Vec<i32>> = vec![vec![i32::MIN; map[0].len()]; map.len()];
    let start = find('S', &map);
    run(&start, &mut map.clone(), &mut distance, 0);

    part1(&map, &distance);
    part2(&map, &distance);
}

fn part1(map: &Vec<Vec<char>>, distance: &Vec<Vec<i32>>) {
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

fn part2(map: &Vec<Vec<char>>, distance: &Vec<Vec<i32>>) {
    let max_len = 20;
    let min_saving = 100; // TODO use 100 for input
    let mut count = 0;
    let mut test: HashMap<i32,i32> = HashMap::new();
    for y in 1..distance.len()-1 {
        println!("{:?}", y);
        for x in 1..distance[y].len()-1 {
            if distance[y][x] != i32::MIN {
                count += count_cheats_starting_at(map, distance, y, x, min_saving, max_len, &mut test);
            }
        }
    }
    for key in test.keys().sorted() {
        println!("{:?} {:?}", test[key], key);
    }

    println!("Part 2: {:?}", count);
}

fn count_cheats_starting_at(map: &Vec<Vec<char>>, distance: &Vec<Vec<i32>>, y: usize, x: usize, min_saving: i32, max_len: i32, test: &mut HashMap<i32,i32>) -> i32 {

    //let cheats = dijkstra(distance, (x, y), max_len);
    let cheats = ignoring_walls(distance, (x, y), max_len);
    let mut count = 0;
    for cheat in cheats {
        if cheat.1 >= min_saving {
            count += 1;
            *test.entry(cheat.1).or_insert(0) += 1;
        }
    }
    // 285 for example1 with 50
    // 1230222 to high
    return count;
}

fn ignoring_walls(distance_from_start: &Vec<Vec<i32>>, start: (usize, usize), max_len: i32) -> HashMap<(usize,usize), i32> {
    let mut result:HashMap<(usize,usize), i32> = HashMap::new();

    for y in 1..distance_from_start.len() as i32-1 {
        for x in 1..distance_from_start[y as usize].len() as i32-1 {
            if distance_from_start[y as usize][x as usize] != i32::MIN {
                let dist = (start.0 as i32-x).abs() + (start.1 as i32-y).abs();
                if dist <= max_len {
                    let saved = distance_from_start[y as usize][x as usize] - distance_from_start[start.1][start.0] - dist;
                    if saved >= 0 {
                        result.insert((x as usize, y as usize), saved);
                    }
                }
            }
        }
    }

    return result;
}


fn dijkstra(distance_from_start: &Vec<Vec<i32>>, start: (usize, usize), max_len: i32) -> HashMap<(usize,usize), i32> {
    let mut distance:HashMap<(usize,usize), i32> = HashMap::new();
    let mut result:HashMap<(usize,usize), i32> = HashMap::new();
    let mut queue: Vec<(usize,usize)> = Vec::new();
    distance.insert(start, 0);
    queue.push(start);

    while queue.len() > 0 {
        let index = find_shortest(&queue, &distance);
        let current = queue.swap_remove(index);

        let current_dist = distance.get(&current).unwrap().clone();
        if current_dist > max_len {
            break;
        }

        if distance_from_start[current.1][current.0] != i32::MIN && current != start {
            let saved = (distance_from_start[current.1][current.0] - distance_from_start[start.1][start.0]) - current_dist;
            //println!("{:?} {:?}", current, saved);
            if !result.contains_key(&current) {
                result.insert(current, saved);
            }
            continue;
        }

        update_distance(distance_from_start, &mut distance, &mut queue, (current.0, current.1+1), current_dist + 1);
        update_distance(distance_from_start, &mut distance, &mut queue, (current.0+1, current.1), current_dist + 1);
        update_distance(distance_from_start, &mut distance, &mut queue, (current.0, current.1.wrapping_sub(1)), current_dist + 1);
        update_distance(distance_from_start, &mut distance, &mut queue, (current.0.wrapping_sub(1), current.1), current_dist + 1);
    }

    return result;
}

fn update_distance(distance_from_start: &Vec<Vec<i32>>, distances: &mut HashMap<(usize, usize), i32>, queue: &mut Vec<(usize, usize)>, node: (usize, usize), distance: i32) {
    if (node.0 >= distance_from_start[0].len()) || (node.1 >= distance_from_start.len()) {
        return;
    }
    match distances.get(&node) {
        None => {
            queue.push(node);
            distances.insert(node, distance);
        }
        Some(old_distance) => {
            if *old_distance > distance {
                queue.push(node);
                distances.insert(node, distance);
            }
        }
    }
}

fn find_shortest(queue: &Vec<(usize,usize)>, dist: &HashMap<(usize,usize), i32>) -> usize {
    let mut min = i32::MAX;
    let mut min_node = 0;
    for i in 0..queue.len() {
        let d = dist.get(&queue[i]).unwrap();
        if *d < min {
            min = *d;
            min_node = i;
        }
    }
    return min_node;
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

fn debug(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    let mut y = 0;
    for row in grid {
        let mut x = 0;
        for cell in row {
            if (x, y) == start {
                print!("1");
            } else if (x, y) == end {
                print!("9");
            } else {
                print!("{}", cell);
            }
            x += 1;
        }
        println!();
        y += 1;
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

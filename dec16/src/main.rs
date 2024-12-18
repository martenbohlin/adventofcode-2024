use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn main() {
    let map = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let start = find('S', &map);
    let (cost, paths) = dijkstra(&map, Direction::East, start);

    part1(cost);
    part2(paths, &map);
}

fn part1(cost: i32) {
    println!("Part 1: {}", cost);
}

fn part2(paths: HashMap<Node, HashSet<Node>>, map: &Vec<Vec<char>>) {
    let end = find('E', map);
    let mut on_any_path: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<Node> = Vec::new();
    queue.push(Node { pos: end, direction: Direction::North });
    queue.push(Node { pos: end, direction: Direction::East });
    queue.push(Node { pos: end, direction: Direction::South });
    queue.push(Node { pos: end, direction: Direction::West });

    while queue.len() > 0 {
        let curr = queue.swap_remove(0);
        on_any_path.insert(curr.pos);
        match paths.get(&curr) {
            None => {},
            Some(prev) => {
                for p in prev {
                    queue.push(p.clone());
                }
            }
        }
    }
    println!("Part 2: {}", on_any_path.len());
}

fn dijkstra(map: &Vec<Vec<char>>, direction: Direction, start: (usize, usize)) -> (i32, HashMap<Node, HashSet<Node>>) {
    let mut distance:HashMap<Node, i32> = HashMap::new();
    let mut queue: Vec<Node> = Vec::new();
    let mut prev: HashMap<Node, HashSet<Node>> = HashMap::new();
    let end = find('E', &map);
    let start_node = Node { pos: start, direction };
    distance.insert(start_node, 0);
    queue.push(start_node);


    let mut min_dist = i32::MAX;
    while queue.len() > 0 {
        let index = find_shortest(&queue, &distance);
        let current = queue.swap_remove(index);

        let current_dist = distance.get(&current).unwrap().clone();
        if current_dist > min_dist {
            break;
        }

        if current.pos == end {
            min_dist = current_dist;
            continue;
        }

        update_cost(map, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: mov(current.direction, current.pos), direction: current.direction }, current_dist + 1);
        update_cost(map, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: current.pos, direction: rotate_left(current.direction) }, current_dist + 1000);
        update_cost(map, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: current.pos, direction: rotate_right(current.direction) }, current_dist + 1000);
    }

    return (min_dist, prev);
}

fn update_cost(map: &Vec<Vec<char>>, dist: &mut HashMap<Node, i32>, prev: &mut HashMap<Node, HashSet<Node>>, queue: &mut Vec<Node>, prev_node: Node, node: Node, new_cost: i32) {
    if map[node.pos.1][node.pos.0] == '#' {
        return;
    }
    match dist.get(&node) {
        None => {
            queue.push(node);
            dist.insert(node, new_cost);
            let mut p = HashSet::new();
            p.insert(prev_node);
            prev.insert(node, p);
        }
        Some(old_cost) => {
            if *old_cost == new_cost {
                prev.entry(node).or_insert_with(||HashSet::new()).insert(prev_node);
                dist.insert(node, new_cost);
            } else if *old_cost > new_cost {
                queue.push(node);
                dist.insert(node, new_cost);
                let mut p = HashSet::new();
                p.insert(prev_node);
                prev.insert(node, p);
            }
        }
    }
}

fn find_shortest(queue: &Vec<Node>, dist: &HashMap<Node, i32>) -> usize {
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

fn mov(direction: Direction, current: (usize, usize)) -> (usize, usize) {
    match direction {
        Direction::North => (current.0, current.1-1),
        Direction::East => (current.0+1, current.1),
        Direction::South => (current.0, current.1+1),
        Direction::West => (current.0-1, current.1),
    }
}

fn rotate_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node {
    pos: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
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


fn read_file(filename: String) -> Vec<Vec<char>> {
    let lines:Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut map:Vec<Vec<char>> = Vec::new();
    for line in lines {
        let l: Vec<char> = line.chars().collect();
        map.push(l);
    }
    return map;
}

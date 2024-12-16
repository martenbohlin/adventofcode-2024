use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let map = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let start = find('S', &map);
    part1(&map, Direction::East, start);
}

fn part1(map: &Vec<Vec<char>>, direction: Direction, start: (usize, usize)) {
    let cost = dijkstra(map, direction, start);
    println!("Part 1: {}", cost);
}

fn dijkstra(map: &Vec<Vec<char>>, direction: Direction, start: (usize, usize)) -> i64 {
    let mut dist:HashMap<Node, i64> = HashMap::new();
    let mut queue: Vec<Node> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '#' {
                dist.insert(Node { pos: (x, y), direction: Direction::North }, i64::MAX);
                queue.push(Node { pos: (x, y), direction: Direction::North });

                dist.insert(Node { pos: (x, y), direction: Direction::East }, i64::MAX);
                queue.push(Node { pos: (x, y), direction: Direction::East });

                dist.insert(Node { pos: (x, y), direction: Direction::South }, i64::MAX);
                queue.push(Node { pos: (x, y), direction: Direction::South });

                dist.insert(Node { pos: (x, y), direction: Direction::West}, i64::MAX);
                queue.push(Node { pos: (x, y), direction: Direction::West });
            }
        }
    }
    let end = find('E', &map);
    dist.insert(Node { pos: start, direction }, 0);


    while queue.len() > 0 {
        if queue.len() % 100 == 0 {
            println!("Q: {:?}", queue.len());
        }
        let index = find_shortest(&queue, &dist);
        let current = queue[index];
        queue.remove(index);
        let current_dist = dist.get(&current).unwrap().clone();

        if current.pos == end {
            return current_dist;
        }

        update_cost(&mut dist, Node { pos: mov(current.direction, current.pos), direction: current.direction }, current_dist + 1);
        update_cost(&mut dist, Node { pos: current.pos, direction: rotate_left(current.direction) }, current_dist + 1000);
        update_cost(&mut dist, Node { pos: current.pos, direction: rotate_right(current.direction) }, current_dist + 1000);
    }
    return i64::MAX;
}

fn update_cost(dist: &mut HashMap<Node, i64>, node: Node, new_cost: i64) {
    match dist.get(&node) {
        None => {}
        Some(d) => {
            if *d > new_cost {
                dist.insert(node, new_cost);
            }
        }
    }
}

fn find_shortest(queue: &Vec<Node>, dist: &HashMap<Node, i64>) -> usize {
    let mut min = i64::MAX;
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

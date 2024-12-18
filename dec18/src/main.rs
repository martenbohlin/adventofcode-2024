use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn main() {
    let corrupted = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let width = env::args().collect::<Vec<String>>()[2].parse::<usize>().unwrap();
    let bytes = env::args().collect::<Vec<String>>()[3].parse::<usize>().unwrap();

    part1(corrupted[0..bytes].to_vec(), width);
    part2(corrupted, width);
}

fn part1(corrupted: Vec<(usize, usize)>, size: usize) {
    let (length,paths) = dijkstra(&corrupted, size, (0, 0), (size -1, size -1));
    debug(corrupted, size, paths);

    println!("Part 1: {:?}", length);
}

fn part2(corrupted: Vec<(usize, usize)>, size: usize) {
    for n in 0..corrupted.len() {
        let sub = corrupted[0..n].to_vec();
        let (length, paths) = dijkstra(&sub, size, (0, 0), (size - 1, size - 1));
        //debug(sub, size, paths);
        if length == i32::MAX {
            println!("Part 2: {:?}", sub[sub.len()-1]);
            break;
        }
    }
}

fn dijkstra(corrupted: &Vec<(usize, usize)>, size: usize, start: (usize, usize), end: (usize, usize)) -> (i32, HashMap<Node, HashSet<Node>>) {
    let mut distance:HashMap<Node, i32> = HashMap::new();
    let mut queue: Vec<Node> = Vec::new();
    let mut prev: HashMap<Node, HashSet<Node>> = HashMap::new();
    let start_node = Node { pos: start };
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
            break;
        }

        update_cost(corrupted, size, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: (current.pos.0+1, current.pos.1)}, current_dist + 1);
        update_cost(corrupted, size, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: (current.pos.0, current.pos.1+1)}, current_dist + 1);
        update_cost(corrupted, size, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: (current.pos.0.wrapping_sub(1), current.pos.1)}, current_dist + 1);
        update_cost(corrupted, size, &mut distance, &mut prev, &mut queue, current,
                    Node { pos: (current.pos.0, current.pos.1.wrapping_sub(1))}, current_dist + 1);
    }

    return (min_dist, prev);
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

fn update_cost(corrupted: &Vec<(usize, usize)>, size: usize, dist: &mut HashMap<Node, i32>, prev: &mut HashMap<Node, HashSet<Node>>, queue: &mut Vec<Node>, prev_node: Node, node: Node, new_cost: i32) {
    if corrupted.contains(&node.pos) || node.pos.0 >= size || node.pos.1 >= size {
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node {
    pos: (usize, usize),
}

fn read_file(filename: String) -> Vec<(usize, usize)> {
    return read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .map(|s| {
            let parts = s.split(',').collect::<Vec<&str>>();
            (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap())
        })
        .collect();  // gather them together into a vector
}

fn debug(corrupted: Vec<(usize, usize)>, size: usize, paths: HashMap<Node, HashSet<Node>>) {
    let mut curr = Node { pos: (size - 1, size - 1) };
    let mut path = HashSet::new();
    while paths.contains_key(&curr) {
        path.insert(curr);
        curr = paths.get(&curr).unwrap().iter().next().unwrap().clone();
    }

    for y in 0..size {
        for x in 0..size {
            if corrupted.contains(&(x, y)) {
                print!("X");
            } else if path.contains(&Node { pos: (x, y) }) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::hash::{Hash,Hasher};
use itertools::Itertools;

fn main() {
    let connections = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let mut computers:HashMap<Computer,HashSet<Computer>> = HashMap::new();
    for (a,b) in connections {
        computers.entry(a.clone()).or_insert(HashSet::new()).insert(b.clone());
        computers.entry(b.clone()).or_insert(HashSet::new()).insert(a.clone());
    }
    part1(&computers);
    part2(&computers);
}

fn part1(computers: &HashMap<Computer,HashSet<Computer>>) {
    let mut groups: HashSet<Network> = HashSet::new();
    for c in computers.keys() {
        if c.starts_with("t") {
            let xs = networks_of_size(computers, c, 3);
            for x in xs {
                groups.insert(x.clone());
            }
        }
    }
    println!("Part 1: {:?}", groups.len());
}

fn part2(computers: &HashMap<Computer,HashSet<Computer>>) {
    let mut largest = Network(HashSet::new());
    for c in computers.keys() {
        if c.starts_with("t") {
            let xs = all_networks(computers, c);
            for x in xs {
                if x.0.len() > largest.0.len() {
                    largest = x;
                }
            }
        }
    }
    let computers = largest.0.iter()
        .sorted()
        .join(",");
    println!("Part 2: {:?}", computers);
}

fn all_networks(computers: &HashMap<Computer,HashSet<Computer>>, base: &Computer) -> HashSet<Network> {
    let mut result:HashSet<Network> = HashSet::new();
    for size in 4..computers.len() {
        let xs = networks_of_size(computers, base, size);
        for x in xs {
            result.insert(x.clone());
        }
    }
    result
}

fn networks_of_size(computers: &HashMap<Computer,HashSet<Computer>>, base: &Computer, size: usize) -> HashSet<Network> {
    let mut result:HashSet<Network> = HashSet::new();
    let possible_networks: Vec<Vec<&Computer>> = computers.get(base).unwrap().iter().combinations(size-1).collect::<Vec<Vec<&Computer>>>();
    'outer: for possible_network in possible_networks {
        for i in 0..(possible_network.len() - 1) {
            let c1 = possible_network[i];
            for j in i + 1..possible_network.len() {
                let c2 = possible_network[j];
                if !computers.get(c1).unwrap().contains(c2) {
                    continue 'outer;
                }
            }
        }
        let mut xs = HashSet::new();
        xs.insert(base.clone());
        for c in possible_network {
            xs.insert(c.clone());
        }
        result.insert(Network(xs));
    }

    result
}

type Computer = String;
#[derive(Debug, Clone)]
struct Network(HashSet<Computer>);

impl PartialEq for Network {
    fn eq(&self, other: &Network) -> bool {
        self.0.is_subset(&other.0) && other.0.is_subset(&self.0)
    }
}

impl Eq for Network {}

impl Hash for Network {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&Computer> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

fn read_file(filename: String) -> Vec<(Computer, Computer)> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| String::from(line))  // make each slice into a string
        .map(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            (Computer::from(parts[0]), Computer::from(parts[1]))
        })  // make each slice into a string
        .collect()  // gather them together into a vector
}

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::hash::{Hash,Hasher};

fn main() {
    let connections = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let mut computers:HashMap<Computer,HashSet<Computer>> = HashMap::new();
    for (a,b) in connections {
        computers.entry(a.clone()).or_insert(HashSet::new()).insert(b.clone());
        computers.entry(b.clone()).or_insert(HashSet::new()).insert(a.clone());
    }
    //for c in computers.keys() {
    //    println!("{}: {:?}", c, computers.get(c).unwrap());
    //}
    part1(&computers);
}

fn part1(computers: &HashMap<Computer,HashSet<Computer>>) {
    let mut groups: HashSet<Network> = HashSet::new();
    for c in computers.keys() {
        if c.starts_with("t") {
            let xs = has_group_of_3(computers, c);
            for x in xs {
                groups.insert(x.clone());
            }
        }
    }
    println!("Part 1: {:?}", groups.len());
}

fn has_group_of_3(computers: &HashMap<Computer,HashSet<Computer>>, base: &Computer) -> HashSet<Network> {
    let mut result:HashSet<Network> = HashSet::new();
    for c1 in computers.get(base).unwrap() {
        for c2 in computers.get(c1).unwrap() {
            if computers.get(c2).unwrap().contains(base) {
                let mut xs = HashSet::new();
                xs.insert(base.clone());
                xs.insert(c1.clone());
                xs.insert(c2.clone());
                result.insert(Network(xs));
            }
        }
    }

    return result;
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

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let (rules, updates) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&rules, &updates);
    part2(&rules, updates);
}

fn part1(rules: &HashMap<i32,HashSet<i32>>, updates: &Vec<Vec<i32>>) {
    let mut sum = 0;
    for update in updates {
        if update_ok(&rules, &update) {
            sum += update[(update.len()-1)/2];
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(rules: &HashMap<i32,HashSet<i32>>, updates: Vec<Vec<i32>>) {
    let mut sum = 0;
    for mut update in updates {
        if !update_ok(&rules, &update) {
            let fixed_update = fix_order(&rules, &mut update);
            sum += fixed_update[(fixed_update.len()-1)/2];
        }
    }
    println!("Part 2: {}", sum);
}

fn fix_order(rules: &HashMap<i32,HashSet<i32>>, update: &mut Vec<i32>) -> Vec<i32> {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 1..update.len() {
            match rules.get(&update[i]) {
                Some(rule) => {
                    for j in 0..i {
                        if rule.contains(&update[j]) { // Found a page that must be after this one
                            update.swap(i, j);
                            changed = true;
                            break;
                        }
                    }
                },
                None => continue,
            }
        }
    }
    return update.clone();
}

fn update_ok(rules: &HashMap<i32,HashSet<i32>>, update: &Vec<i32>) -> bool {
    for i in 1..update.len() {
        match  rules.get(&update[i]){
            Some(rule) => {
                for j in 0..i {
                    if rule.contains(&update[j]) {
                        return false;
                    }
                }
            },
            None => continue,
        }
    }
    return true;
}

fn read_file(filename: String) -> (HashMap<i32,HashSet<i32>>, Vec<Vec<i32>>) {
    let all_lines:Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let mut rules_done = false;
    for line in all_lines {
        if line.len() == 0 {
            rules_done = true;
        } else if !rules_done {
            let (first, later) = parse_rule(&line);
            rules.entry(first).or_insert(HashSet::new()).insert(later);
        } else {
            updates.push(parse_update(&line));
        }

    }

    return (rules, updates);
}

fn parse_rule(line: &String) -> (i32, i32) {
    let parts = line.split("|").collect::<Vec<&str>>();
    (parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
}

fn parse_update(line: &String) -> Vec<i32> {
    line.split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

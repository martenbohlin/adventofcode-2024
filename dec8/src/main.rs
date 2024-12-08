use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let (antennas, width, height) = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&antennas, width, height);
    part2(&antennas, width, height);
}

fn part1(antennas: &HashMap<char,Vec<(i32,i32)>>, width: i32, height: i32) {
    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for (_freq, positions) in antennas {
        add_antinodes(&mut antinodes, positions, width, height);
    }
    println!("Part 1: {}", antinodes.len());
}

fn part2(antennas: &HashMap<char,Vec<(i32,i32)>>, width: i32, height: i32) {
    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for (_freq, positions) in antennas {
        add_antinodes_p2(&mut antinodes, positions, width, height);
    }
    println!("Part 2: {}", antinodes.len());
}

fn add_antinodes(antinodes: &mut HashSet<(i32, i32)>, positions: &Vec<(i32, i32)>, width: i32, height: i32) {
    let nr_antennas = positions.len();
    for i in 0..nr_antennas {
        for j in i+1..nr_antennas {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let p1 = (x1 - dx, y1 - dy);
            if within_city(p1, width, height) {
                antinodes.insert(p1);
            }
            let p2 = (x2 + dx, y2 + dy);
            if within_city(p2, width, height) {
                antinodes.insert(p2);
            }
        }
    }
}

fn add_antinodes_p2(antinodes: &mut HashSet<(i32, i32)>, positions: &Vec<(i32, i32)>, width: i32, height: i32) {
    let nr_antennas = positions.len();
    for i in 0..nr_antennas {
        for j in i+1..nr_antennas {
            let (mut x1, mut y1) = positions[i];
            let (mut x2, mut y2) = positions[j];
            let dx = x2 - x1;
            let dy = y2 - y1;

            while within_city((x1, y1), width, height) {
                antinodes.insert((x1,y1));
                x1 = x1 - dx;
                y1 = y1 - dy;
            }

            while within_city((x2, y2), width, height) {
                antinodes.insert((x2, y2));
                x2 = x2 + dx;
                y2 = y2 + dy;
            }
        }
    }
}


fn within_city(p: (i32, i32), width: i32, height: i32) -> bool {
    return p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height;
}

fn read_file(filename: String) -> (HashMap<char,Vec<(i32,i32)>>, i32, i32) {
    let grid: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut result = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let frequency = grid[y][x];
            if frequency != '.' {
                result.entry(frequency).or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }
    return (result, grid[0].len() as i32, grid.len() as i32);
}

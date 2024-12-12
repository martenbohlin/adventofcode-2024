use std::fs::read_to_string;
use std::env;

fn main() {
    let mut map = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let areas = find_areas(&mut map);
    part1(&areas);
}

fn part1(areas: &Vec<Area>) {
    let mut sum = 0;
    for area in areas {
        let a = area;
        sum += a.perimeter * a.area;
    }
    println!("Part 1: {}", sum);
}

fn find_areas(map: &mut Vec<Vec<char>>) -> Vec<Area> {
    let mut result = Vec::new();
    // iterate over the map and find all areas
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                let mut area = Area { plant: map[y][x], plots: Vec::new(), area: 0, perimeter: 0 };
                find_area(map, &mut area, &(x,y));
                result.push(area);
            }
        }
    }
    return result;
}

fn find_area(map: &mut Vec<Vec<char>>, area: &mut Area, coordinate: &(usize, usize)) {
    let x = coordinate.0;
    let y = coordinate.1;
    if map[y][x] != area.plant {
        return;
    }
    map[y][x] = '.';
    add_to_area(area, coordinate);
    if y > 0 {
        find_area(map, area, &(x, y-1));
    }
    if y < map.len()-1 {
        find_area(map, area, &(x, y+1));
    }
    if x > 0 {
        find_area(map, area, &(x-1, y));
    }
    if x < map[y].len()-1 {
        find_area(map, area, &(x+1, y));
    }
}

fn add_to_area(area: &mut Area, coordinate: &(usize, usize)) {
    area.plots.push(coordinate.clone());
    area.area += 1;
    area.perimeter += 4;
    let x = coordinate.0;
    let y = coordinate.1;
    if area.plots.contains(&(x.wrapping_sub(1), y)) {
        area.perimeter -= 2;
    }
    if area.plots.contains(&(x.wrapping_add(1), y)) {
        area.perimeter -= 2;
    }
    if area.plots.contains(&(x, y.wrapping_sub(1))) {
        area.perimeter -= 2;
    }
    if area.plots.contains(&(x, y.wrapping_add(1))) {
        area.perimeter -= 2;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Area {
    plant: char,
    plots: Vec<(usize,usize)>,
    area: i32,
    perimeter: i32,
}


fn read_file(filename: String) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect()  // gather them together into a vector
}

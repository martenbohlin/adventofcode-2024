use std::fs::read_to_string;
use std::env;

fn main() {
    let disk = parse_input();

    part1(&disk);
}

fn part1(input_disk: &Vec<Option<MyFile>>) {
    let mut disk = input_disk.clone();
    let mut next_free = find_next_free(&disk, 0);
    for i in (next_free..disk.len()).rev() {
        match disk[i] {
            Some(f) => {
                if next_free < i {
                    disk[next_free] = Some(f);
                    disk[i] = None;
                    next_free = find_next_free(&disk, next_free);
                }
            },
            None => {
            },
        }
    }

    println!("Part 1: {}", checksum_disk(&disk));
}

fn find_next_free(disk: &Vec<Option<MyFile>>, start: usize) -> usize {
    let mut i = start + 1;
    while i < disk.len() {
        match disk[i as usize] {
            Some(_) => i += 1,
            None => return i,
        }
    }
    return i as usize;
}

fn checksum_disk(disk: &Vec<Option<MyFile>>) -> i64 {
    let mut sum:i64 = 0;
    let mut i = 0;
    for file in disk {
        match file {
            Some(f) => {
                sum += (f.id * i) as i64;
            },
            None => continue,
        }
        i = i + 1;
    }
    return sum;
}

fn parse_input() -> Vec<Option<MyFile>> {
    let input: Vec<char> = read_to_string(env::args().collect::<Vec<String>>()[1].clone()).unwrap().chars().collect();
    let mut result:Vec<Option<MyFile>> = Vec::new();
    let mut file_id = 0;
    for i in 0..=input.len()/2 {
        let c: char = input[(i * 2) as usize];
        let size:i32 = c.to_digit(10).unwrap() as i32;
        let file = MyFile{id: file_id, size: size};
        file_id = file_id + 1;
        for _ in 0..size {
            result.push(Some(file));
        }
        if (i * 2 + 1) < input.len() {
            let c: char = input[(i * 2 + 1) as usize];
            let size:i32 = c.to_digit(10).unwrap() as i32;
            for _ in 0..size {
                result.push(None);
            }
        }
    }
    return result;
}

#[derive(Debug, Copy, Clone)]
struct MyFile {
    id: i32,
    size: i32,
}

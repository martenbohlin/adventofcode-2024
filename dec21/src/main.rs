use std::env;
use std::fs::read_to_string;
use cached::proc_macro::cached;

static NUM_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['X', '0', 'A'],
];


static DIRECTION_KEYPAD: [[char; 3]; 2] = [
    ['X', '^', 'A'],
    ['<', 'v', '>'],
];


fn main() {
    let codes = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&codes);
    part2(&codes);
}

fn part1(codes: &Vec<Vec<char>>) {
    println!("Part 1: {}", solve(codes, 2));
}

fn part2(codes: &Vec<Vec<char>>) {
    println!("Part 2: {}", solve(codes, 25));
}

fn solve(codes: &Vec<Vec<char>>, nr_directional: usize) -> i64 {
    let mut sum_of_complexity = 0;
    for code in codes {
        let mut moves = 0_i64;
        let mut prev_button = 'A';
        for button in code {
            // For each digit to enter, find the optimal number of buttons i have to push
            moves += best_move_for_number_pad(prev_button, *button, nr_directional + 1);
            prev_button = *button;
        }

        let code_value = code[0..code.len() - 1].into_iter().collect::<String>().parse::<i64>().unwrap();
        println!("   {} * {}", moves, code_value);
        sum_of_complexity += code_value * moves;
    }
    sum_of_complexity
}

fn best_move_for_number_pad(prev_button: char, number_button: char, nr_directional: usize) ->  i64 {
    let keypad = NUM_KEYPAD;
    let possible_moves = move_to(keypad, find_button(keypad, &prev_button), &number_button).unwrap();

    best_move(nr_directional, &possible_moves)
}

#[cached]
fn best_move_for_direction_pad(prev_button: char, button: char, depth: usize) ->  i64 {
    if depth == 0 { // This is me pushing a button, that's just 1 move
        return 1;
    }

    let keypad = DIRECTION_KEYPAD;
    let possible_moves = move_to(keypad, find_button(keypad, &prev_button), &button).unwrap();

    best_move(depth, &possible_moves)
}

fn best_move(depth: usize, possible_moves: &Vec<Vec<char>>) -> i64 {
    let mut result = i64::MAX;
    for moves in possible_moves {
        let mut top_level_buttons = 0;
        let mut prev_button = 'A';
        for button in moves {
            top_level_buttons += best_move_for_direction_pad(prev_button, *button, depth - 1);
            prev_button = *button;
        }
        result = result.min(top_level_buttons);
    }

    result
}

// Find all possible ways to move from a position on the key-pad to a button
fn move_to<const W: usize, const H: usize>(keypad: [[char;W];H], pos: (usize, usize), button: &char) -> Option<Vec<Vec<char>>> {
    let target = find_button(keypad, button);

    let dy = pos.1 as i32 - target.1 as i32;
    let dx = pos.0 as i32 - target.0 as i32;
    if dx == 0 && dy == 0 {
        return Some([['A'].to_vec()].to_vec());
    }
    if keypad[pos.1][pos.0] == 'X' {
        return None;
    }
    let mut result: Vec<Vec<char>> = Vec::new();
    if dy != 0 {
        match move_to(keypad, (pos.0, if dy < 0 { pos.1+1 } else { pos.1-1 }), &button) {
            Some(mut moves) => {
                for mut movement in &mut moves {
                    let mut x: Vec<char> = Vec::new();
                    x.push(if dy < 0 { 'v' } else { '^' });
                    x.append(&mut movement);
                    result.push(x);
                }
            }
            None => {
            }
        }
    }

    if dx != 0 {
        match move_to(keypad, (if dx < 0 { pos.0 + 1 } else { pos.0 - 1 }, pos.1), &button) {
            Some(mut moves) => {
                for mut movement in &mut moves {
                    let mut x: Vec<char> = Vec::new();
                    x.push(if dx < 0 { '>' } else { '<' });
                    x.append(&mut movement);
                    result.push(x);
                }
            }
            None => {
            }
        }
    }
    Some(result)
}

// Locate a button on the keypad
fn find_button<const W: usize, const H: usize>(keypad: [[char;W];H], button: &char) -> (usize, usize) {
    for y in 0..keypad.len() {
        for x in 0..keypad[y].len() {
            if keypad[y][x] == *button {
                return (x, y);
            }
        }
    }
    panic!("Button not found");
}


fn read_file(filename: String) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.chars().collect())  // make each slice into a string
        .collect()  // gather them together into a vector
}

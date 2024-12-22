use once_cell::sync::OnceCell;
use std::env;
use std::fs::read_to_string;
use cached::proc_macro::cached;

static NUM_KEYPAD: OnceCell<Vec<Vec<char>>> = OnceCell::new();

static DIRECTION_KEYPAD: OnceCell<Vec<Vec<char>>> = OnceCell::new();

fn main() {
    NUM_KEYPAD.set(
        [
            ['7', '8', '9'].to_vec(),
            ['4', '5', '6'].to_vec(),
            ['1', '2', '3'].to_vec(),
            ['X', '0', 'A'].to_vec(),
        ].to_vec()
    ).unwrap();

    DIRECTION_KEYPAD.set(
        [
            ['X', '^', 'A'].to_vec(),
            ['<', 'v', '>'].to_vec(),
        ].to_vec()
    ).unwrap();

    let codes = read_file(env::args().collect::<Vec<String>>()[1].clone());
    part1(&codes);
    part2(&codes);
}

fn part1(codes: &Vec<Vec<char>>) {
    let mut sum = 0;
    for code in codes {
        let mut movement3 = 0_i64;
        let mut prev_button = 'A';
        for button in code {
            let moves = best_move_for_number_pad(prev_button, *button, 2+1);
            prev_button = *button;
            movement3 += moves
        }

        let code_value = code[0..code.len()-1].into_iter().collect::<String>().parse::<i64>().unwrap();
        println!("   {} * {}", movement3, code_value);
        sum += code_value * movement3;
    }
    println!("Part 1: {}", sum);
}

fn part2(codes: &Vec<Vec<char>>) {
    let mut sum = 0;
    for code in codes {
        let mut movement = 0_i64;
        let mut prev_button = 'A';
        for button in code {
            let moves = best_move_for_number_pad(prev_button, *button, 25+1);
            prev_button = *button;
            movement += moves;
        }

        let code_value = code[0..code.len()-1].into_iter().collect::<String>().parse::<i64>().unwrap();
        println!("   {} * {}", movement, code_value);
        sum += code_value * movement;
    }
    // 423797863237514 to high
    println!("Part 2: {}", sum);
}

fn best_move_for_number_pad(prev_button: char, number_button: char, nr_directional: usize) ->  i64 {
    let keypad = NUM_KEYPAD.get().unwrap();
    let possible_moves = move_to(keypad, find_button(keypad, &prev_button), &number_button).unwrap();

    let mut result = i64::MAX;
    for moves in &possible_moves {
        let mut top_level_buttons = 0;
        let mut prev_button = 'A';
        for button in moves {
            top_level_buttons += best_move_for_direction_pad(prev_button, *button, nr_directional-1);
            prev_button = *button;
        }
        result = result.min(top_level_buttons);
    }

    result
}

#[cached]
fn best_move_for_direction_pad(prev_button: char, button: char, depth: usize) ->  i64 {
    if depth == 0 {
        return 1;
    }

    let keypad = DIRECTION_KEYPAD.get().unwrap();
    let possible_moves = move_to(keypad, find_button(keypad, &prev_button), &button).unwrap();

    let mut result = i64::MAX;
    for moves in &possible_moves {
        let mut top_level_buttons = 0;
        let mut prev_button = 'A';
        for button in moves {
            top_level_buttons += best_move_for_direction_pad(prev_button, *button, depth-1);
            prev_button = *button;

        }
        result = result.min(top_level_buttons);
    }

    result
}

fn directional_robot(start_at: (usize, usize), moves: &Vec<char>) -> Vec<Vec<char>> {
    //let mut pos2: (usize, usize) = find_button(&DIRECTION_KEYPAD, &'A');
    let pos2 = start_at;
    let mut movements2: Vec<Vec<char>> = Vec::new();
    let mut possible_combinations: Vec<Vec<char>> = Vec::new();
    for button2 in moves {
        let next_moves = move_to(DIRECTION_KEYPAD.get().unwrap(), pos2, &button2).unwrap();
        if possible_combinations.is_empty() {
            possible_combinations = next_moves;
        } else {
            let mut next_combinations = Vec::new();
            for combination in &possible_combinations {
                for next_move in &next_moves {
                    let mut y = combination.clone();
                    y.append(&mut next_move.clone());
                    next_combinations.push(y);
                }
            }
            possible_combinations = next_combinations;
        }
        movements2.append(&mut possible_combinations);
    }
    movements2
}

fn move_to(keypad: &Vec<Vec<char>>, pos: (usize, usize), button: &char) -> Option<Vec<Vec<char>>> {
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

fn find_button(keypad: &Vec<Vec<char>>, button: &char) -> (usize, usize) {
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

use std::env;
use std::fs::read_to_string;


fn main() {
    let codes = read_file(env::args().collect::<Vec<String>>()[1].clone());

    let num_keypad = [
        ['7', '8', '9'].to_vec(),
        ['4', '5', '6'].to_vec(),
        ['1', '2', '3'].to_vec(),
        ['X', '0', 'A'].to_vec(),
    ].to_vec();

    let direction_keypad = [
        ['X', '^', 'A'].to_vec(),
        ['<', 'v', '>'].to_vec(),
    ].to_vec();

    part1(&codes, &num_keypad, &direction_keypad);
}

fn part1(codes: &Vec<Vec<char>>, num_keypad: &Vec<Vec<char>>, direction_keypad: &Vec<Vec<char>>) {
    let mut sum = 0;
    for code in codes {
        let mut pos: (usize, usize) = find_button(num_keypad, &'A');
        let mut movement3: Vec<char> = Vec::new();
        for button in code {
            let mut moves = best_move(num_keypad, direction_keypad, &mut pos, button);
            movement3.append(&mut moves);
        }

        let code_value = code[0..code.len()-1].into_iter().collect::<String>().parse::<i32>().unwrap();
        println!("   {} * {}", movement3.len(), code_value);
        sum += code_value * movement3.len() as i32;
    }
    println!("Part 1: {}", sum);
}

fn best_move(num_keypad: &Vec<Vec<char>>, direction_keypad: &Vec<Vec<char>>, mut pos: &mut (usize, usize), button: &char) ->  Vec<char> {
    let movements1 = move_to(num_keypad, &mut pos, button).unwrap();

    let mut pos2: (usize, usize) = find_button(direction_keypad, &'A');
    let mut movements2: Vec<Vec<char>> = Vec::new();
    for moves in &movements1 {
        let mut possible_combinations: Vec<Vec<char>> = Vec::new();
        for button2 in moves {
            let next_moves = move_to(direction_keypad, &mut pos2, &button2).unwrap();
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
        }
        movements2.append(&mut possible_combinations);
    }
    //for mut movement2 in &movements2 {
    //    println!("2: {:?}", movement2.clone().into_iter().collect::<String>());
    //}
    //println!("2  {:?} {:?}", pos2, button);

    let mut pos3: (usize, usize) = find_button(direction_keypad, &'A');
    let mut movements3: Vec<Vec<char>> = Vec::new();
    for moves in &movements2 {
        let mut possible_combinations: Vec<Vec<char>> = Vec::new();
        for button3 in moves {
            let next_moves = move_to(direction_keypad, &mut pos3, &button3).unwrap();
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
        }
        movements3.append(&mut possible_combinations);
    }
    //for mut movement3 in &movements3 {
    //    println!("3: {:?}", movement3.clone().into_iter().collect::<String>());
    //}
    //println!("3  {:?} {:?}", pos3, button);

    let mut min_len = i32::MAX;
    let mut result: Vec<char> = Vec::new();
    for m3 in &movements3 {
        let len = m3.len() as i32;
        if len < min_len {
            min_len = len;
            result = m3.clone();
        }
    }

    return result
}

fn move_to(keypad: &Vec<Vec<char>>, pos: &mut (usize, usize), button: &char) -> Option<Vec<Vec<char>>> {
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
        match move_to(keypad, &mut(pos.0, if dy < 0 { pos.1+1 } else { pos.1-1 }), &button) {
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
        match move_to(keypad, &mut(if dx < 0 { pos.0 + 1 } else { pos.0 - 1 }, pos.1), &button) {
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
    pos.0 = target.0;
    pos.1 = target.1;
    return Some(result);
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

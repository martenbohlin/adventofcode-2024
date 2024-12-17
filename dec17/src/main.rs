use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;

fn main() {
    let state = read_file(env::args().collect::<Vec<String>>()[1].clone());
    println!("{:?}", state);
    part1(&state);
    part2(state.0, state.1, state.2, &state.3);
}

fn part1(state: &(i64, i64, i64, Vec<i64>)) {
    let output = execute(state.0, state.1, state.2, &state.3);
    print!("Part 1: ");
    for o in output {
        print!("{},", o);
    }
    println!();
}

fn part2(_a: i64, b: i64, c:i64, program: &Vec<i64>) {
    print!("Part 2: ");
    //for a in 5170000000..100000000000 {
    //
    // Started with big steps Iterated and looked at the output of more and more digits matching in the end
    // Start 35184000000000 --- 281475000000000  diff 246291000000000 right number of digits

    //for a in (0..281475000000000).step_by(10000000) {
    //for a in (136897000000000..136907000000000).step_by(100) {
    //for a in (109019150000000..136902148400000).step_by(100) {
    for a in 109019476328500..109019489009700 {
        let output = execute(a, b, c, program);
        if output == *program {
            println!("Part 2: {}", a);
            break;
        }
        //println!("a: {} {:?}", a, output);
    }

    // To high 136902148098203
}

fn execute(ax: i64, bx: i64, cx:i64, program: &Vec<i64>) -> Vec<i64> {
    let mut a = ax;
    let mut b = bx;
    let mut c = cx;
    let mut ip = 0;
    let mut output: Vec<i64> = Vec::new();
    while ip < program.len() {
        match program[ip] {
            0 => { // adv
                a = a / 2_i64.pow(combo(program[ip + 1], (a, b, c)).try_into().unwrap());
                ip += 2;
            },
            1 => { // bxl
                b = b ^ program[ip + 1];
                ip += 2;
            },
            2 => { // bst
                b = combo(program[ip + 1], (a, b, c)) % 8;
                ip += 2;
            },
            3 => { // jnz
                if a == 0 {
                    ip += 2;
                } else {
                    ip = program[ip + 1] as usize;
                }
            },
            4 => { // bxc
                b = b ^ c;
                ip += 2;
            },
            5 => { // out
                output.push(combo(program[ip + 1], (a, b, c)) % 8);
                ip += 2;
            },
            6 => { // bdv
                b = a / 2_i64.pow(combo(program[ip + 1], (a, b, c)).try_into().unwrap());
                ip += 2;
            },
            7 => { // cdv
                c = a / 2_i64.pow(combo(program[ip + 1], (a, b, c)).try_into().unwrap());
                ip += 2;
            },
            _ => panic!("Invalid opcode")
        }
    }
    output
}

fn combo(opcode: i64, (a,b,c): (i64, i64, i64)) -> i64 {
    match opcode {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid opcode")
    }
}

fn read_file(filename: String) -> (i64, i64, i64, Vec<i64>) {
    /*
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0

     */
    let lines:Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let a = lines[0].split_whitespace().collect::<Vec<&str>>()[2].parse::<i64>().unwrap();
    let b = lines[1].split_whitespace().collect::<Vec<&str>>()[2].parse::<i64>().unwrap();
    let c = lines[2].split_whitespace().collect::<Vec<&str>>()[2].parse::<i64>().unwrap();
    let program = lines[4].split_whitespace().collect::<Vec<&str>>()[1].split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    return (a, b, c, program);
}

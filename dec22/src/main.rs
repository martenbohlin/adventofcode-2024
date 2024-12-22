use std::env;
use std::fs::read_to_string;

fn main() {
    let start = read_file(env::args().collect::<Vec<String>>()[1].clone());
    let series = part1(&start);
    part2(series);
}

fn part1(start: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sum = 0_i64;
    let mut result = Vec::new();
    for x in start {
        let mut serie = Vec::new();
        let mut y = *x;
        serie.push(y);
        for _i in 0..2000 {
            y = next(&y);
            serie.push(y);
        }
        //println!("{}: {}", x, y);
        sum += y as i64;
        result.push(serie);
    }
    println!("Part 1: {}", sum);
    return result
}

fn part2(series: Vec<Vec<i64>>) {
    let deltas = calculate_deltas(series);
    let mut max = 0;
    for a1 in -9..=9 {
        println!("a1: {}", a1);
        for a2 in -9..=9 {
            for a3 in -9..=9 {
                for a4 in -9..=9 {
                    let bananas = enter_auction(&deltas, a1, a2, a3, a4);
                    if bananas > max {
                        max = bananas;
                        println!("{} {} {} {} {}", a1, a2, a3, a4, max);
                    }
                }
            }
        }
    }
    println!("Part 2: {:?}", max);
    //println!("Part 2: {:?}", enter_auction(&deltas, -2,1,-1,3));
}

fn enter_auction(deltas: &Vec<Vec<(i64, i64)>>, a: i64, b: i64, c: i64, d: i64) -> i64 {
    let mut result = 0;
    for delta in deltas {
        for i in 0..delta.len()-3 {
            if delta[i].1 == a && delta[i+1].1 == b && delta[i+2].1 == c && delta[i+3].1 == d {
                result += delta[i + 3].0;
                break;
            }
        }
    }
    return result;
}

fn calculate_deltas(series: Vec<Vec<i64>>) -> Vec<Vec<(i64, i64)>> {
    let mut result: Vec<Vec<(i64, i64)>> = Vec::new();
    for serie in series {
        let mut deltas: Vec<(i64, i64)> = Vec::new();
        for i in 0..serie.len() - 1 {
            let before = serie[i] % 10;
            let after = serie[i + 1] % 10;
            deltas.push((after, after - before));
        }
        result.push(deltas);
    }
    return result;
}

fn next(x: &i64) -> i64 {
    let prune = 16777216;
    let mut result = ((x*64) ^x) % prune;
    result = ((result/32) ^ result) % prune;
    result = ((result*2048) ^ result) % prune;
    return result;
}


fn read_file(filename: String) -> Vec<i64> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|line| line.parse::<i64>().unwrap())  // make each slice into a string
        .collect()  // gather them together into a vector
}

use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut input_path = "input.txt";
    if args.len() > 1 && args[1] == "test" || args.len() > 2 && args[2] == "test" {
        input_path = "input_test.txt";
    }
    let input = fs::read_to_string(input_path).unwrap();
    if args.len() < 2 || args[1] != "part2" {
        println!("Part 1: {}", part1(&input));
        return;
    }
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            result += process_pattern(&pattern);
            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    result += process_pattern(&pattern);
    result
}

fn process_pattern(pattern: &Vec<Vec<char>>) -> u32 {
    for i in 0..pattern.len() - 1 {
        let mut is_mirror = true;
        let mut j: i32 = i as i32;
        while j >= 0 {
            let k: usize = i + i - j as usize + 1;
            if k >= pattern.len() {
                break;
            }
            if pattern[j as usize] != pattern[k] {
                is_mirror = false;
                break;
            }
            j -= 1;
        }
        if is_mirror {
            eprintln!("Found mirror at line {}", i);
            return (i + 1) as u32 * 100;
        }
    }
    for i in 0..pattern[0].len() - 1 {
        let mut is_mirror = true;
        let mut j: i32 = i as i32;
        while j >= 0 {
            let k: usize = i + i - j as usize + 1;
            if k >= pattern[0].len() {
                break;
            }
            for l in 0..pattern.len() {
                if pattern[l][j as usize] != pattern[l][k] {
                    is_mirror = false;
                    break;
                }
            }
            j -= 1;
        }
        if is_mirror {
            eprintln!("Found mirror at column {}", i);
            return i as u32 + 1;
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            result += process_pattern_part2(&pattern);
            pattern.clear();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    result += process_pattern_part2(&pattern);
    result
}

fn process_pattern_part2(pattern: &Vec<Vec<char>>) -> u32 {
    for i in 0..pattern.len() - 1 {
        let mut is_mirror = true;
        let mut j: i32 = i as i32;
        let mut mismatch = 0;
        while j >= 0 {
            let k: usize = i + i - j as usize + 1;
            if k >= pattern.len() {
                break;
            }
            for l in 0..pattern[0].len() {
                if pattern[j as usize][l] != pattern[k][l] {
                    mismatch += 1;
                    if mismatch > 1 {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if !is_mirror {
                break;
            }
            j -= 1;
        }
        if is_mirror && mismatch == 1 {
            eprintln!("Found mirror at line {}", i);
            return (i + 1) as u32 * 100;
        }
    }
    for i in 0..pattern[0].len() - 1 {
        let mut is_mirror = true;
        let mut j: i32 = i as i32;
        let mut mismatch = 0;
        while j >= 0 {
            let k: usize = i + i - j as usize + 1;
            if k >= pattern[0].len() {
                break;
            }
            for l in 0..pattern.len() {
                if pattern[l][j as usize] != pattern[l][k] {
                    mismatch += 1;
                    if mismatch > 1 {
                        is_mirror = false;
                        break;
                    }
                }
            }
            if !is_mirror {
                break;
            }
            j -= 1;
        }
        if is_mirror && mismatch == 1 {
            eprintln!("Found mirror at column {}", i);
            return i as u32 + 1;
        }
    }
    0
}

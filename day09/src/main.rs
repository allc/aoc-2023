use std::{fs, vec};

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

fn part1(input: &str) -> i32 {
    let input: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
    }).collect();
    eprintln!("Input: {:?}", input);
    let mut sum = 0;
    for history in input {
        sum += get_next_value(history);
    }
    sum
}

fn get_next_value(history: Vec<i32>) -> i32 {
    let mut history_: Vec<Vec<i32>> = vec![history];
    loop {
        let last = history_.last().unwrap();
        if last.iter().all(|x| *x == 0) {
            break;
        }
        let mut next: Vec<i32> = Vec::new();
        for i in 0..last.len() - 1 {
            next.push(last[i + 1] - last[i]);
        }
        history_.push(next);
    }
    eprintln!("History: {:?}", history_);
    let mut next = 0;
    for i in 2..history_.len() + 1 {
        next += *history_[history_.len() - i].last().unwrap();
        println!("Next: {}", next);
    }
    next
}

fn part2(input: &str) -> i32 {
    let input: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
    }).collect();
    eprintln!("Input: {:?}", input);
    let mut sum = 0;
    for mut history in input {
        history.reverse();
        sum += get_next_value(history);
    }
    sum
}

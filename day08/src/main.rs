use std::{fs, collections::HashMap};
use num::integer::lcm;

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

struct Node {
    label: String,
    left: String,
    right: String,
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let mut nodes = HashMap::new();
    for line in lines {
        let label = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        nodes.insert(label.clone(), Node { label, left, right });
    }
    let mut n_steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let node = nodes.get(current).unwrap();
        let instruction = instructions[n_steps % instructions.len()];
        if instruction == 'L' {
            current = node.left.as_str();
        } else {
            current = node.right.as_str();
        }
        n_steps += 1;
    }
    n_steps as u32
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    let mut nodes = HashMap::new();
    for line in lines {
        let label = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        nodes.insert(label.clone(), Node { label, left, right });
    }
    let mut starting_nodes = nodes.keys().filter(|k| k.ends_with("A")).collect::<Vec<&String>>();
    let mut result: u64 = 1;
    for starting_node in starting_nodes {
        let mut n_steps = 0;
        let mut current = starting_node;
        while !current.ends_with('Z') {
            let node = nodes.get(current).unwrap();
            let instruction = instructions[n_steps % instructions.len()];
            if instruction == 'L' {
                current = &node.left;
            } else {
                current = &node.right;
            }
            n_steps += 1;
        }
        result = lcm(result, n_steps as u64);
    }
    result
}

fn is_all_current_end_with_z(current_nodes: &Vec<&String>) -> bool {
    for node in current_nodes {
        if !node.ends_with("Z") {
            return false;
        }
    }
    true
}

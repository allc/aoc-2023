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
    let mut lines = input.lines();
    let times: Vec<u32> = lines.next().unwrap()[5..]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines.next().unwrap()[9..]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    eprintln!("Times: {:?}", times);
    eprintln!("Distances: {:?}", distances);
    let mut answer = 1;
    for i in 0..times.len() {
        let mut num_ways = 0;
        let record = distances[i];
        for j in 1..times[i] {
            if j * (times[i] - j) > record {
                num_ways += 1;
            }
        }
        eprintln!("{}: {}", i, num_ways);
        answer *= num_ways;
    }
    answer
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time: u64 = lines.next().unwrap()[5..].replace(" ", "").parse().unwrap();
    let distance: u64 = lines.next().unwrap()[9..].replace(" ", "").parse().unwrap();
    eprintln!("Times: {:?}", time);
    eprintln!("Distances: {:?}", distance);
    let mut num_ways = 0;
    for j in 1..time {
        if j * (time - j) > distance {
            num_ways += 1;
        }
    }
    num_ways
}

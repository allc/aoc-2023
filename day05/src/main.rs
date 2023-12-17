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

#[derive(Debug)]
struct Map {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

fn part1(input: &str) -> u64 {
    let mut current = -1;
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            seeds = line[7..]
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            eprintln!("seeds: {:?}", seeds);
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if line.ends_with("map:") {
            current += 1;
            maps.push(Vec::new());
            continue;
        }
        let mut parts = line.split(" ");
        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range = parts.next().unwrap().parse::<u64>().unwrap();
        maps[current as usize].push(Map {
            dest_start,
            source_start,
            range,
        });
    }
    for maps_ in &maps {
        eprintln!("maps: {:?}", maps_);
    }
    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let mut source = seed;
        for maps_ in &maps {
            source = map(source, maps_);
        }
        eprintln!("Location: {}", source);
        lowest_location = lowest_location.min(source);
    }
    lowest_location
}

fn map(source: u64, maps: &Vec<Map>) -> u64 {
    for map in maps {
        if source >= map.source_start && source < map.source_start + map.range {
            return map.dest_start + source - map.source_start;
        }
    }
    source
}

fn part2(input: &str) -> u64 {
    let mut current = -1;
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Map>> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            seeds = line[7..]
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            eprintln!("seeds: {:?}", seeds);
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if line.ends_with("map:") {
            current += 1;
            maps.push(Vec::new());
            continue;
        }
        let mut parts = line.split(" ");
        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let range = parts.next().unwrap().parse::<u64>().unwrap();
        maps[current as usize].push(Map {
            dest_start,
            source_start,
            range,
        });
    }
    for maps_ in &maps {
        eprintln!("maps: {:?}", maps_);
    }
    let mut lowest_location = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            let mut source = j;
            for maps_ in &maps {
                source = map(source, maps_);
            }
            lowest_location = lowest_location.min(source);
            if j % 1000000 == 0 {
                eprint!("Seed: {}\r", j);
            }
        }
    }
    eprintln!();
    lowest_location
}

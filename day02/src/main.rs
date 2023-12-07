use std::{fs, collections::HashMap, cmp::{min, max}};

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
    let BAG = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;
    for line in input.lines() {
        let line = line.split(": ").collect::<Vec<&str>>();
        let game_id = line[0].split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        eprintln!("Game ID: {}", game_id);
        let sets = line[1].split("; ").collect::<Vec<&str>>();
        eprintln!("Sets: {:?}", sets);
        let mut is_possible = true;
        for set in sets {
            let cubes = set.split(", ").collect::<Vec<&str>>();
            eprintln!("Cubes: {:?}", cubes);
            for cube in cubes {
                let cube = cube.split(" ").collect::<Vec<&str>>();
                let color = cube[1];
                let count = cube[0].parse::<u32>().unwrap();
                if BAG[color] < count {
                    is_possible = false;
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        if is_possible {
            sum += game_id;
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.split(": ").collect::<Vec<&str>>();
        eprintln!("{}", line[0]);
        let sets = line[1].split("; ").collect::<Vec<&str>>();
        eprintln!("Sets: {:?}", sets);
        let mut bag = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in sets {
            let cubes = set.split(", ").collect::<Vec<&str>>();
            eprintln!("Cubes: {:?}", cubes);
            for cube in cubes {
                let cube = cube.split(" ").collect::<Vec<&str>>();
                let color = cube[1];
                let count = cube[0].parse::<u32>().unwrap();
                bag.insert(color, max(*bag.get(color).unwrap(), count));
            }
        }
        let power = bag["red"] * bag["green"] * bag["blue"];
        sum += power;
    }
    sum
}

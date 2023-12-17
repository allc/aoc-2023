use std::{fs, collections::HashMap};

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
    let mut platform: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        platform.push(line.chars().collect::<Vec<char>>());
    }
    let mut load: u32 = 0;
    for x in 0..platform[0].len() {
        let mut empty_space = 0;
        for y in 0..platform.len() {
            if platform[y][x] == '.' {
                empty_space += 1;
            } else if platform[y][x] == '#' {
                empty_space = 0;
            } else {
                load += platform.len() as u32 - y as u32 + empty_space;
            }
        }
    }
    load
}

fn part2(input: &str) -> u32 {
    let mut platform: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        platform.push(line.chars().collect::<Vec<char>>());
    }
    let mut seen_platforms: HashMap<String, usize> = HashMap::new();
    for i in 0..1000000000 {
        cycle(&mut platform);
        let platform_string = print_platform(&platform);
        if seen_platforms.contains_key(&platform_string) {
            let cycle_start = seen_platforms.get(&platform_string).unwrap();
            let cycle_length = i - cycle_start;
            let cycle_index = (1000000000 - i - 1) % cycle_length;
            for _ in 0..cycle_index {
                cycle(&mut platform);
            }
            break;
        }
        seen_platforms.insert(platform_string, i);
    }
    eprintln!("{}", print_platform(&platform));
    get_load(&platform)
}

fn get_load(platform: &Vec<Vec<char>>) -> u32 {
    let mut load: u32 = 0;
    for y in 0..platform.len() {
        for x in 0..platform[0].len() {
            if platform[y][x] == 'O' {
                load += platform.len() as u32 - y as u32;
            }
        }
    }
    load
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
}

fn print_platform(platform: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    for line in platform {
        for c in line {
            output.push(*c);
        }
        output.push('\n');
    }
    output
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    for x in 0..platform[0].len() {
        let mut empty_space = 0;
        for y in 0..platform.len() {
            if platform[y][x] == '.' {
                empty_space += 1;
            } else if platform[y][x] == '#' {
                empty_space = 0;
            } else {
                platform[y][x] = '.';
                platform[y - empty_space][x] = 'O';
            }
        }
    }
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
    for y in 0..platform.len() {
        let mut empty_space = 0;
        for x in 0..platform[0].len() {
            if platform[y][x] == '.' {
                empty_space += 1;
            } else if platform[y][x] == '#' {
                empty_space = 0;
            } else {
                platform[y][x] = '.';
                platform[y][x - empty_space] = 'O';
            }
        }
    }
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
    for x in 0..platform[0].len() {
        let mut empty_space = 0;
        for y in (0..platform.len()).rev() {
            if platform[y][x] == '.' {
                empty_space += 1;
            } else if platform[y][x] == '#' {
                empty_space = 0;
            } else {
                platform[y][x] = '.';
                platform[y + empty_space][x] = 'O';
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
    for y in 0..platform.len() {
        let mut empty_space = 0;
        for x in (0..platform[0].len()).rev() {
            if platform[y][x] == '.' {
                empty_space += 1;
            } else if platform[y][x] == '#' {
                empty_space = 0;
            } else {
                platform[y][x] = '.';
                platform[y][x + empty_space] = 'O';
            }
        }
    }
}

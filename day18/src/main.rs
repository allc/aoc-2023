use std::{fs, collections::HashSet};

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
    let mut plan: Vec<(char, u32)> = Vec::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let direction = line[0].chars().next().unwrap();
        let distance = line[1].parse::<u32>().unwrap();
        plan.push((direction, distance));
    }
    eprintln!("{:?}", plan);
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (direction, distance) in &plan {
        match direction {
            'R' => x += *distance as i32,
            'L' => x -= *distance as i32,
            'U' => y -= *distance as i32,
            'D' => y += *distance as i32,
            _ => panic!("Unknown direction {}", direction),
        }
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    eprintln!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    let height = max_y - min_y + 3;
    let width = max_x - min_x + 3;
    let mut x = 1 - min_x;
    let mut y = 1 - min_y;
    let mut grid = vec![vec![0; width as usize]; height as usize];
    for (direction, distance) in plan {
        match direction {
            'R' => {
                for _ in 0..distance {
                    x += 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'L' => {
                for _ in 0..distance {
                    x -= 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'U' => {
                for _ in 0..distance {
                    y -= 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'D' => {
                for _ in 0..distance {
                    y += 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            _ => panic!("Unknown direction {}", direction),
        }
    }
    // eprintln!("{:?}", grid);
    let mut seen_tiles: HashSet<(u32, u32)> = HashSet::new();
    let mut stack: Vec<(u32, u32)> = Vec::new();
    stack.push((0, 0));
    while !stack.is_empty() {
        let current_tile = stack.pop().unwrap();
        if seen_tiles.contains(&current_tile) {
            continue;
        }
        seen_tiles.insert(current_tile);
        let (y, x) = current_tile;
        if x > 0 && grid[y as usize][(x - 1) as usize] == 0 {
            stack.push((y, x - 1));
        }
        if x < (width - 1).try_into().unwrap() && grid[y as usize][(x + 1) as usize] == 0 {
            stack.push((y, x + 1));
        }
        if y > 0 && grid[(y - 1) as usize][x as usize] == 0 {
            stack.push((y - 1, x));
        }
        if y < (height - 1).try_into().unwrap() && grid[(y + 1) as usize][x as usize] == 0 {
            stack.push((y + 1, x));
        }
    }
    let n_not_lava = seen_tiles.len();
    (width * height - n_not_lava as i32) as u32
}

fn part2(input: &str) -> u32 {
    let directions = vec!['R', 'D', 'L', 'U'];
    let mut plan: Vec<(char, u32)> = Vec::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let code = line[2];
        let direction = directions[code[7..8].parse::<usize>().unwrap()];
        let distance = u32::from_str_radix(&code[2..7], 16).unwrap();
        plan.push((direction, distance));
    }
    eprintln!("{:?}", plan);
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (direction, distance) in &plan {
        match direction {
            'R' => x += *distance as i32,
            'L' => x -= *distance as i32,
            'U' => y -= *distance as i32,
            'D' => y += *distance as i32,
            _ => panic!("Unknown direction {}", direction),
        }
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    eprintln!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    let height = max_y - min_y + 3;
    let width = max_x - min_x + 3;
    let mut x = 1 - min_x;
    let mut y = 1 - min_y;
    let mut grid:Vec<Vec<i8>> = vec![vec![0; width as usize]; height as usize];
    for (direction, distance) in plan {
        match direction {
            'R' => {
                for _ in 0..distance {
                    x += 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'L' => {
                for _ in 0..distance {
                    x -= 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'U' => {
                for _ in 0..distance {
                    y -= 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            'D' => {
                for _ in 0..distance {
                    y += 1;
                    grid[y as usize][x as usize] = 1;
                }
            },
            _ => panic!("Unknown direction {}", direction),
        }
    }
    // eprintln!("{:?}", grid);
    let mut seen_tiles: HashSet<(u32, u32)> = HashSet::new();
    let mut stack: Vec<(u32, u32)> = Vec::new();
    stack.push((0, 0));
    while !stack.is_empty() {
        let current_tile = stack.pop().unwrap();
        if seen_tiles.contains(&current_tile) {
            continue;
        }
        seen_tiles.insert(current_tile);
        let (y, x) = current_tile;
        if x > 0 && grid[y as usize][(x - 1) as usize] == 0 {
            stack.push((y, x - 1));
        }
        if x < (width - 1).try_into().unwrap() && grid[y as usize][(x + 1) as usize] == 0 {
            stack.push((y, x + 1));
        }
        if y > 0 && grid[(y - 1) as usize][x as usize] == 0 {
            stack.push((y - 1, x));
        }
        if y < (height - 1).try_into().unwrap() && grid[(y + 1) as usize][x as usize] == 0 {
            stack.push((y + 1, x));
        }
    }
    let n_not_lava = seen_tiles.len();
    (width * height - n_not_lava as i32) as u32
}

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
    let tiles: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    eprintln!("{:?}", tiles);
    let mut start_y = 0;
    let mut start_x = 0;
    for (y, row) in tiles.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_y = y;
                start_x = x;
                break;
            }
        }
    }
    eprintln!("Start: {}, {}", start_y, start_x);
    let mut last_y = start_y;
    let mut last_x = start_x;
    let (mut current_y, mut current_x) = find_first(&tiles, start_y, start_x);
    eprintln!("First: {} {}, {}", tiles[current_y][current_x], current_y, current_x);
    let mut count = 1;
    while current_x != start_x || current_y != start_y {
        if tiles[current_y][current_x] == '|' {
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == '-' {
            if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'L' {
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'J' {
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == '7' {
            if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'F' {
            if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else {
            panic!("Unknown tile: {}", tiles[current_y][current_x]);
        }
        eprintln!("Current: {} {}, {}", tiles[current_y][current_x], current_y, current_x);
        count += 1;
    }
    count / 2
}

fn find_first(tiles: &Vec<Vec<char>>, start_y: usize, start_x: usize) -> (usize, usize) {
    if start_y > 0 {
        if "|7F".contains(tiles[start_y - 1][start_x]) {
            return (start_y - 1, start_x);
        }
    }
    if start_y < tiles.len() - 1 {
        if "|JL".contains(tiles[start_y + 1][start_x]) {
            return (start_y + 1, start_x);
        }
    }
    if start_x > 0 {
        if "-LF".contains(tiles[start_y][start_x - 1]) {
            return (start_y, start_x - 1);
        }
    }
    if start_x < tiles[0].len() - 1 {
        if "-J7".contains(tiles[start_y][start_x + 1]) {
            return (start_y, start_x + 1);
        }
    }
    panic!("No first pipe found");
}

fn part2(input: &str) -> i64 {
    let tiles: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // eprintln!("{:?}", tiles);
    let mut start_y = 0;
    let mut start_x = 0;
    for (y, row) in tiles.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_y = y;
                start_x = x;
                break;
            }
        }
    }
    eprintln!("Start: {}, {}", start_y, start_x);
    let mut last_y = start_y;
    let mut last_x = start_x;
    let (mut current_y, mut current_x) = find_first(&tiles, start_y, start_x);
    eprintln!("First: {} {}, {}", tiles[current_y][current_x], current_y, current_x);
    let mut vertices = vec![(start_y as i64, start_x as i64)];
    let mut b = 1;
    while current_x != start_x || current_y != start_y {
        b += 1;
        if tiles[current_y][current_x] == '|' {
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == '-' {
            if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'L' {
            vertices.push((current_y as i64, current_x as i64));
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'J' {
            vertices.push((current_y.try_into().unwrap(), current_x.try_into().unwrap()));
            if current_y - 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y -= 1;
            } else if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == '7' {
            vertices.push((current_y as i64, current_x as i64));
            if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else if current_x - 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x -= 1;
            } else {
                panic!("No next pipe found");
            }
        } else if tiles[current_y][current_x] == 'F' {
            vertices.push((current_y as i64, current_x as i64));
            if current_y + 1 != last_y {
                last_y = current_y;
                last_x = current_x;
                current_y += 1;
            } else if current_x + 1 != last_x {
                last_y = current_y;
                last_x = current_x;
                current_x += 1;
            } else {
                panic!("No next pipe found");
            }
        } else {
            panic!("Unknown tile: {}", tiles[current_y][current_x]);
        }
        // eprintln!("Current: {} {}, {}", tiles[current_y][current_x], current_y, current_x);
    }
    eprintln!("{:?}", vertices);
    let mut A: i64 = 0;
    for i in 0..vertices.len() {
        A += vertices[i].1 * (vertices[(i + 1) % vertices.len()].0 - vertices[((i as i64 - 1 + vertices.len() as i64) % vertices.len() as i64) as usize].0);
    }
    let i = A.abs() + 2 - b + 1;
    i / 2 as i64
}

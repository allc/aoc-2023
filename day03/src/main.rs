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
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for i in 0..schematic.len() {
        let mut num = 0;
        let mut num_start = 0;
        let mut num_end = 0;
        let mut last = '.';
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                if !last.is_alphanumeric() {
                    num_start = j;
                }
                num = num * 10 + schematic[i][j].to_digit(10).unwrap();
                num_end = j;
            } else {
                if last.is_numeric() {
                    if is_part_number(&schematic, i, num_start, num_end) {
                        eprintln!("Part number: {}", num);
                        sum += num;
                    }
                    num = 0;
                }
            }
            last = schematic[i][j];
        }
        if last.is_numeric() {
            if is_part_number(&schematic, i, num_start, num_end) {
                eprintln!("Part number: {}", num);
                sum += num;
            }
        }
    }
    sum
}

fn is_part_number(schematic: &Vec<Vec<char>>, i: usize, num_start: usize, num_end: usize) -> bool {
    let mut start = num_start;
    if num_start > 0 {
        if is_symbol(schematic[i][num_start - 1]) {
            return true;
        }
        start = num_start - 1;
    }
    let mut end = num_end;
    if num_end < schematic[i].len() - 1 {
        if is_symbol(schematic[i][num_end + 1]) {
            return true;
        }
        end = num_end + 1;
    }
    for j in start..end + 1{
        if i > 0 && is_symbol(schematic[i - 1][j]) {
            return true;
        }
        if i < schematic.len() - 1 && is_symbol(schematic[i + 1][j]) {
            return true;
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn part2(input: &str) -> u32 {
    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut gears: HashMap<Position, Vec<u32>> = HashMap::new();
    for i in 0..schematic.len() {
        let mut num = 0;
        let mut num_start = 0;
        let mut num_end = 0;
        let mut last = '.';
        for j in 0..schematic[i].len() {
            if schematic[i][j].is_numeric() {
                if !last.is_alphanumeric() {
                    num_start = j;
                }
                num = num * 10 + schematic[i][j].to_digit(10).unwrap();
                num_end = j;
            } else {
                if last.is_numeric() {
                    let adjacent_gears = find_adjacent_gears(&schematic, i, num_start, num_end);
                    for gear in adjacent_gears {
                        gears.entry(gear).or_insert(Vec::new()).push(num);
                    }
                    num = 0;
                }
            }
            last = schematic[i][j];
        }
        if last.is_numeric() {
            let adjacent_gears = find_adjacent_gears(&schematic, i, num_start, num_end);
            for gear in adjacent_gears {
                gears.entry(gear).or_insert(Vec::new()).push(num);
            }
        }
    }
    eprintln!("Gears: {:?}", gears);
    for (_, values) in gears {
        if values.len() == 2 {
            let product = values[0] * values[1];
            sum += product;
        }
    }
    sum
}

fn find_adjacent_gears(schematic: &Vec<Vec<char>>, i: usize, num_start: usize, num_end: usize) -> Vec<Position> {
    let mut gears = Vec::new();
    let mut start = num_start;
    if num_start > 0 {
        if schematic[i][num_start - 1] == '*' {
            gears.push(Position { x: i, y: num_start - 1 });
        }
        start = num_start - 1;
    }
    let mut end = num_end;
    if num_end < schematic[i].len() - 1 {
        if schematic[i][num_end + 1] == '*' {
            gears.push(Position { x: i, y: num_end + 1 });
        }
        end = num_end + 1;
    }
    for j in start..end + 1{
        if i > 0 && schematic[i - 1][j] == '*' {
            gears.push(Position { x: i - 1, y: j });
        }
        if i < schematic.len() - 1 && schematic[i + 1][j] == '*' {
            gears.push(Position { x: i + 1, y: j });
        }
    }
    gears
}
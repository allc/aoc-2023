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
    let mut sum = 0;
    for line in input.lines() {
        let mut num = 0;
        for c in line.chars() {
            if c.is_numeric() {
                num += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                num += c.to_digit(10).unwrap();
                break;
            }
        }
        eprintln!("Calibration value: {}", num);
        sum += num;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut i = 0;
        while i < line.len() {
            if line[i..i + 1].chars().next().unwrap().is_numeric() {
                first_digit = line[i..].chars().next().unwrap().to_digit(10).unwrap();
                break;
            }
            let mut found = false;
            for (j, digit) in digits.iter().enumerate() {
                if line[i..].starts_with(digit) {
                    first_digit = (j + 1) as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            i += 1;
        }
        let mut i: i32 = line.len() as i32 - 1;
        while i >= 0 {
            if line[i as usize..].chars().next().unwrap().is_numeric() {
                last_digit = line[i as usize..].chars().next().unwrap().to_digit(10).unwrap();
                break;
            }
            let mut found = false;
            for (j, digit) in digits.iter().enumerate() {
                if line[i as usize..].starts_with(digit) {
                    last_digit = (j + 1) as u32;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            i -= 1;
        }
        let num = first_digit * 10 + last_digit;
        eprintln!("Calibration value: {}", num);
        sum += num;
    }
    sum
}

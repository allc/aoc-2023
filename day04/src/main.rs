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
    let mut sum = 0;
    for line in input.lines() {
        let line = line.split(": ").collect::<Vec<&str>>();
        eprintln!("{:?}", line);
        let numbers = line[1].split(" | ").collect::<Vec<&str>>();
        eprintln!("Numbers: {:?}", numbers);
        let winning_numbers: HashSet<&str> = HashSet::from_iter(numbers[0].split_whitespace());
        eprintln!("Winning number: {:?}", winning_numbers);
        let numbers_have = numbers[1].split_whitespace().collect::<Vec<&str>>();
        eprintln!("Number have: {:?}", numbers_have);
        let mut num_matches = 0;
        for number in numbers_have {
            if winning_numbers.contains(number) {
                num_matches += 1;
            }
        }
        eprintln!("Number of matches: {}", num_matches);
        /*
        1: 1
        2: 2
        3: 4
        4: 8
        5: 16 */
        if num_matches == 0 {
            continue;
        }
        sum += u32::pow(2, num_matches - 1);
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut num_cards: Vec<u32> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if num_cards.len() <= i {
            num_cards.push(0);
        }
        num_cards[i] += 1;
        let line = line.split(": ").collect::<Vec<&str>>();
        eprintln!("{:?}", line);
        let numbers = line[1].split(" | ").collect::<Vec<&str>>();
        eprintln!("Numbers: {:?}", numbers);
        let winning_numbers: HashSet<&str> = HashSet::from_iter(numbers[0].split_whitespace());
        eprintln!("Winning number: {:?}", winning_numbers);
        let numbers_have = numbers[1].split_whitespace().collect::<Vec<&str>>();
        eprintln!("Number have: {:?}", numbers_have);
        let mut num_matches = 0;
        for number in numbers_have {
            if winning_numbers.contains(number) {
                num_matches += 1;
            }
        }
        eprintln!("Number of matches: {}", num_matches);
        for j in i + 1..i + num_matches + 1 {
            if num_cards.len() <= j {
                num_cards.push(0);
            }
            num_cards[j] += num_cards[i];
        }
    }
    num_cards.iter().sum()
}

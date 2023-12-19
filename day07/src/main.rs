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

#[derive(Debug)]
struct HandBid {
    hand: Vec<char>,
    type_: u32,
    bid: u32,
}

fn part1(input: &str) -> u32 {
    let mut handbids: Vec<HandBid> = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_at(line.find(" ").unwrap());
        handbids.push(HandBid {
            hand: hand.chars().collect(),
            type_: get_type(hand),
            bid: bid[1..bid.len()].parse::<u32>().unwrap(),
        });
    }
    let card_order = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    handbids.sort_by(|a, b| {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.hand[i] != b.hand[i] {
                    return card_order.iter().position(|&c| c == a.hand[i]).unwrap().cmp(&card_order.iter().position(|&c| c == b.hand[i]).unwrap());
                }
            }
        }
        b.type_.cmp(&a.type_)
    });
    eprintln!("{:?}", handbids);
    let n_hands = handbids.len();
    let mut total_winnings = 0;
    for (i, handbid) in handbids.iter().enumerate() {
        total_winnings += handbid.bid * (n_hands - i) as u32;
    }
    total_winnings
}

fn get_type(hand: &str) -> u32 {
    let mut count: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        let counter = count.entry(c).or_insert(0);
        *counter += 1;
    }
    let mut count_count = vec![0; 6];
    for (_, v) in count {
        count_count[v as usize] += 1;
    }
    if count_count[5] == 1 { // Five of a kind
        return 6;
    }
    if count_count[4] == 1 { // Four of a kind
        return 5;
    }
    if count_count[3] == 1 && count_count[2] == 1 { // Full house
        return 4;
    }
    if count_count[3] == 1 { // Three of a kind
        return 3;
    }
    if count_count[2] == 2 { // Two pairs
        return 2;
    }
    if count_count[2] == 1 { // One pair
        return 1;
    }
    0 // High card
}

fn part2(input: &str) -> u32 {
    let mut handbids: Vec<HandBid> = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_at(line.find(" ").unwrap());
        handbids.push(HandBid {
            hand: hand.chars().collect(),
            type_: get_type_part2(hand),
            bid: bid[1..bid.len()].parse::<u32>().unwrap(),
        });
    }
    let card_order = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    handbids.sort_by(|a, b| {
        if a.type_ == b.type_ {
            for i in 0..5 {
                if a.hand[i] != b.hand[i] {
                    return card_order.iter().position(|&c| c == a.hand[i]).unwrap().cmp(&card_order.iter().position(|&c| c == b.hand[i]).unwrap());
                }
            }
        }
        b.type_.cmp(&a.type_)
    });
    eprintln!("{:?}", handbids);
    let n_hands = handbids.len();
    let mut total_winnings = 0;
    for (i, handbid) in handbids.iter().enumerate() {
        total_winnings += handbid.bid * (n_hands - i) as u32;
    }
    total_winnings
}

fn get_type_part2(hand: &str) -> u32 {
    let mut count: HashMap<char, u32> = HashMap::new();
    let mut j_count = 0;
    for c in hand.chars() {
        if c == 'J' {
            j_count += 1;
            continue;
        }
        let counter = count.entry(c).or_insert(0);
        *counter += 1;
    }
    if j_count == 5 { // Five of a kind
        return 6;
    }
    let mut max_key = count.keys().next().unwrap().clone();
    for (k, v) in &count {
        max_key = if *v > count[&max_key] { *k } else { max_key };
    }
    count.entry(max_key).and_modify(|v| *v += j_count);
    let mut count_count = vec![0; 6];
    for (_, v) in count {
        count_count[v as usize] += 1;
    }
    if count_count[5] == 1 { // Five of a kind
        return 6;
    }
    if count_count[4] == 1 { // Four of a kind
        return 5;
    }
    if count_count[3] == 1 && count_count[2] == 1 { // Full house
        return 4;
    }
    if count_count[3] == 1 { // Three of a kind
        return 3;
    }
    if count_count[2] == 2 { // Two pairs
        return 2;
    }
    if count_count[2] == 1 { // One pair
        return 1;
    }
    0 // High card
}
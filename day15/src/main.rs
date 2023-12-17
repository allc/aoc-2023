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
    let steps = input.trim().split(',').collect::<Vec<&str>>();
    eprintln!("{:?}", steps);
    for step in steps {
        sum += hash(step);
    }
    sum
}

fn hash(step: &str) -> u32 {
    let mut current_value = 0;
    for char in step.chars() {
        let ascii_code = char as u32;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn part2(input: &str) -> u32 {
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    let steps = input.trim().split(',').collect::<Vec<&str>>();
    for step in steps {
        if step.ends_with('-') {
            for box_ in &mut boxes {
                box_.retain(|x| x.label != step[0..step.len() - 1].to_string());
            }
        } else {
            let lens = Lens {
                label: step[0..step.len() - 2].to_string(),
                focal_length: step[step.len() - 1..step.len()].parse::<u32>().unwrap(),
            };
            let hash = hash(&lens.label);
            let box_ = &mut boxes[hash as usize];
            let mut has_label = false;
            for (i, lens_) in box_.iter().enumerate() {
                if lens_.label == lens.label {
                    box_[i].focal_length = lens.focal_length;
                    has_label = true;
                    break;
                }
            }
            if !has_label {
                box_.push(lens);
            }
        }
    }
    eprintln!("Boxes: {:?}", boxes);
    let mut sum = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (j, lens) in box_.iter().enumerate() {
            sum += ((i + 1) * (j + 1)) as u32 * lens.focal_length;
        }
    }
    sum
}

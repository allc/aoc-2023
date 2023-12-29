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

#[derive(Debug, PartialEq)]
enum Op {
    LT,
    GT,
    None,
}

#[derive(Debug, PartialEq)]
struct Rule {
    op: Op,
    property: String,
    value: u32,
    outcome: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut workflows: Vec<Workflow> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (workflow_name, workflow_content) = line.split_once("{").unwrap();
        let workflow_content = workflow_content.strip_suffix("}").unwrap();
        let workflow_content = workflow_content.split(",");
        let mut rules = Vec::new();
        for rule in workflow_content {
            if rule.contains(':') {
                let (condition, outcome) = rule.split_once(':').unwrap();
                let mut operator = Op::None;
                if condition.chars().nth(1).unwrap() == '<' {
                    operator = Op::LT;
                } else if condition.chars().nth(1).unwrap() == '>' {
                    operator = Op::GT;
                }
                let mut property = "none";
                property = &condition[0..1];
                let value = condition
                    .chars()
                    .skip(2)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                rules.push(Rule {
                    op: operator,
                    property: property.to_string(),
                    value,
                    outcome: outcome.to_string(),
                });
            } else {
                rules.push(Rule {
                    op: Op::None,
                    property: "none".to_string(),
                    value: 0,
                    outcome: rule.to_string(),
                });
            }
        }
        workflows.push(Workflow {
            name: workflow_name.to_string(),
            rules,
        });
        eprintln!("Workflow: {:?}", workflows.last().unwrap());
    }
    let mut parts: Vec<HashMap<&str, u32>> = Vec::new();
    for line in lines {
        let mut part: HashMap<&str, u32> = HashMap::new();
        let part_info = line[1..line.len() - 1].split(",");
        for info in part_info {
            let (key, value) = info.split_once("=").unwrap();
            part.insert(key, value.parse::<u32>().unwrap());
        }
        eprintln!("Part: {:?}", part);
        parts.push(part);
    }
    let mut sum = 0;
    for part in parts {
        let mut current_workflow_name = "in";
        loop {
            if current_workflow_name == "R" {
                break;
            }
            if current_workflow_name == "A" {
                eprintln!("Part accepted: {:?}", part);
                sum += part.keys().map(|k| part[k]).sum::<u32>();
                break;
            }
            let current_workflow = workflows
                .iter()
                .find(|w| w.name == *current_workflow_name)
                .unwrap();
            for rule in &current_workflow.rules {
                if rule.property == "none" {
                    current_workflow_name = &rule.outcome;
                    break;
                } else {
                    let value = part[&rule.property.to_string().to_lowercase().as_str()];
                    if rule.op == Op::LT && value < rule.value {
                        current_workflow_name = &rule.outcome;
                        break;
                    } else if rule.op == Op::GT && value > rule.value {
                        current_workflow_name = &rule.outcome;
                        break;
                    }
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    0
}

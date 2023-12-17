use std::{fs, collections::HashMap, hash::Hash};

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
        let (record, contiguous) = line.split_at(line.find(' ').unwrap());
        let record: Vec<char> = record.chars().collect();
        let contiguous: Vec<u32> = contiguous.trim_start().split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        eprintln!("{:?} {:?}", record, contiguous);
        let mut n_arrangements = 0;
        let mut stack: Vec<Vec<char>> = Vec::new();
        stack.push(Vec::new());
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            if current.len() == record.len() {
                if check_arrangement(&current, &contiguous) {
                    n_arrangements += 1;
                }
                continue;
            }
            let mut next1 = current.clone();
            if record[current.len()] != '?' {
                next1.push(record[current.len()]);
                stack.push(next1);
            } else {
                next1.push('#');
                stack.push(next1);
                let mut next2 = current.clone();
                next2.push('.');
                stack.push(next2);
            }
        }
        sum += n_arrangements;
    }
    sum
}

fn check_arrangement(arrangement: &Vec<char>, contiguous: &Vec<u32>) -> bool {
    let mut contiguous_i = 0;
    let mut contiguous_n = 0;
    for c in arrangement {
        if c == &'#' {
            contiguous_n += 1;
        } else {
            if contiguous_n != 0 {
                if contiguous_i >= contiguous.len() {
                    return false;
                }
                if contiguous[contiguous_i] != contiguous_n {
                    return false;
                }
                contiguous_i += 1;
                contiguous_n = 0;
            }
        }
    }
    if contiguous_n != 0 {
        if contiguous_i >= contiguous.len() {
            return false;
        }
        if contiguous[contiguous_i] != contiguous_n {
            return false;
        }
        contiguous_i += 1;
    }
    if contiguous_i != contiguous.len() {
        return false;
    }
    true
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let (record, contiguous) = line.split_at(line.find(' ').unwrap());
        let mut record: Vec<char> = record.chars().collect();
        let mut contiguous: Vec<u32> = contiguous.trim_start().split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let record_ = record.clone();
        let contiguous_ = contiguous.clone();
        for _ in 0..5 - 1 {
            record.push('?');
            record.append(record_.clone().as_mut());
            contiguous.append(contiguous_.clone().as_mut());
        }
        eprintln!("{:?} {:?}", record, contiguous);
        eprintln!("Length: {}", record.len());
        let mut db: HashMap<Problem, u64> = HashMap::new();
        sum += get_num_arrangements(&mut db, Problem {
            record,
            contiguous,
        });
        // eprintln!("{:?}", db);
    }
    sum
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Problem {
    record: Vec<char>,
    contiguous: Vec<u32>,
}

fn get_num_arrangements(db: &mut HashMap<Problem, u64>, problem: Problem) -> u64 {
    if db.contains_key(&problem) {
        return *db.get(&problem).unwrap();
    }
    if problem.record.len() == 0 {
        if problem.contiguous.len() == 0 {
            return 1;
        }
        return 0;
    }
    if problem.record[0] == '.' {
        let n_arrangement = get_num_arrangements(db, Problem {
            record: problem.record[1..].to_vec(),
            contiguous: problem.contiguous.clone(),
        });
        db.insert(problem, n_arrangement);
        return n_arrangement;
    }
    if problem.record[0] == '#' {
        if problem.contiguous.len() == 0 {
            return 0;
        }
        if problem.contiguous[0] == 1 {
            if problem.record.len() > 1 && problem.record[1] == '#' {
                return 0;
            } else if problem.record.len() > 1 && problem.record[1] == '?' {
                let mut record = problem.record[1..].to_vec();
                record[0] = '.';
                return get_num_arrangements(db, Problem {
                    record,
                    contiguous: problem.contiguous[1..].to_vec(),
                });
            } else {
                return get_num_arrangements(db, Problem {
                    record: problem.record[1..].to_vec(),
                    contiguous: problem.contiguous[1..].to_vec(),
                });
            }
        } else {
            if problem.record.len() < 2 {
                return 0;
            }
            if problem.record[1] == '.' {
                return 0;
            }
            if problem.record[1] == '#' {
                let mut contiguous = problem.contiguous.clone();
                contiguous[0] -= 1;
                return get_num_arrangements(db, Problem {
                    record: problem.record[1..].to_vec(),
                    contiguous,
                });
            }
            let mut record = problem.record[1..].to_vec();
            record[0] = '#';
            let mut contiguous = problem.contiguous.clone();
            contiguous[0] -= 1;
            return get_num_arrangements(db, Problem {
                record,
                contiguous,
            });
        }
    }
    let mut record1 = problem.record.clone();
    record1[0] = '.';
    let n_arrangements1 = get_num_arrangements(db, Problem {
        record: record1,
        contiguous: problem.contiguous.clone(),
    });
    let mut record2 = problem.record.clone();
    record2[0] = '#';
    let n_arrangements2 = get_num_arrangements(db, Problem {
        record: record2,
        contiguous: problem.contiguous.clone(),
    });
    return n_arrangements1 + n_arrangements2;
}

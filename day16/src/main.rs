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

struct LightSource {
    y: usize,
    x: usize,
    direction: char,
}

fn part1(input: &str) -> u32 {
    let contraption: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    eprintln!("{:?}", contraption);
    let n_rows = contraption.len();
    let n_cols = contraption[0].len();
    eprintln!("Rows: {}, Cols: {}", n_rows, n_cols);
    let mut grid: Vec<Vec<Vec<char>>> = Vec::new();
    for _ in 0..n_rows {
        let mut row: Vec<Vec<char>> = Vec::new();
        for _ in 0..n_cols {
            row.push(Vec::new());
        }
        grid.push(row);
    }
    let mut light_sources: Vec<LightSource> = Vec::new();
    if ".-".contains(contraption[0][0]) {
        light_sources.push(LightSource {
            y: 0,
            x: 0,
            direction: '>',
        });
        grid[0][0].push('>');
    } else if "|\\".contains(contraption[0][0]) {
        light_sources.push(LightSource {
            y: 0,
            x: 0,
            direction: 'v',
        });
        grid[0][0].push('v');
    } else if contraption[0][0] == '/' {
        return 1;
    }
    while light_sources.len() > 0 {
        let current_light_source = light_sources.pop().unwrap();
        let next_light_sources = get_next_lightsources(&current_light_source, &contraption);
        for next_light_source in next_light_sources {
            let mut is_in_grid = false;
            for light_source in &grid[next_light_source.y][next_light_source.x] {
                if light_source == &next_light_source.direction {
                    is_in_grid = true;
                    break;
                }
            }
            if !is_in_grid {
                grid[next_light_source.y][next_light_source.x].push(next_light_source.direction);
                light_sources.push(next_light_source);
            }
        }
    }
    eprintln!("{:?}", grid);
    let mut n_energized = 0;
    for row in grid {
        for cell in row {
            if cell.len() > 0 {
                n_energized += 1;
            }
        }
    }
    n_energized
}

fn get_next_lightsources(
    current_light_source: &LightSource,
    contraption: &Vec<Vec<char>>,
) -> Vec<LightSource> {
    let mut next_light_sources: Vec<LightSource> = Vec::new();
    if current_light_source.direction == '>' {
        if current_light_source.x + 1 < contraption[0].len() {
            let x = current_light_source.x + 1;
            let y = current_light_source.y;
            if contraption[y][x] == '.' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            } else if contraption[y][x] == '-' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            } else if contraption[y][x] == '|' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            } else if contraption[y][x] == '/' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
            } else if contraption[y][x] == '\\' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            }
        }
    } else if current_light_source.direction == '<' {
        if current_light_source.x > 0 {
            let x = current_light_source.x - 1;
            let y = current_light_source.y;
            if contraption[y][x] == '.' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
            } else if contraption[y][x] == '-' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
            } else if contraption[y][x] == '|' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            } else if contraption[y][x] == '/' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            } else if contraption[y][x] == '\\' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
            }
        }
    } else if current_light_source.direction == '^' {
        if current_light_source.y > 0 {
            let x = current_light_source.x;
            let y = current_light_source.y - 1;
            if contraption[y][x] == '.' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
            } else if contraption[y][x] == '-' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            } else if contraption[y][x] == '|' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '^',
                });
            } else if contraption[y][x] == '/' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            } else if contraption[y][x] == '\\' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
            }
        }
    } else if current_light_source.direction == 'v' {
        if current_light_source.y + 1 < contraption.len() {
            let x = current_light_source.x;
            let y = current_light_source.y + 1;
            if contraption[y][x] == '.' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            } else if contraption[y][x] == '-' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            } else if contraption[y][x] == '|' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: 'v',
                });
            } else if contraption[y][x] == '/' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '<',
                });
            } else if contraption[y][x] == '\\' {
                next_light_sources.push(LightSource {
                    y,
                    x,
                    direction: '>',
                });
            }
        }
    }
    next_light_sources
}

struct EntryPoint {
    y: usize,
    x: usize,
    direction: char,
}

fn part2(input: &str) -> u32 {
    let contraption: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    eprintln!("{:?}", contraption);
    let n_rows = contraption.len();
    let n_cols = contraption[0].len();
    eprintln!("Rows: {}, Cols: {}", n_rows, n_cols);
    let mut entry_points: Vec<EntryPoint> = Vec::new();
    for i in 0..n_rows {
        entry_points.push(EntryPoint {
            y: i,
            x: 0,
            direction: '>',
        });
        entry_points.push(EntryPoint {
            y: i,
            x: n_cols - 1,
            direction: '<',
        });
    }
    for i in 0..n_cols {
        entry_points.push(EntryPoint {
            y: 0,
            x: i,
            direction: 'v',
        });
        entry_points.push(EntryPoint {
            y: n_rows - 1,
            x: i,
            direction: '^',
        });
    }
    let mut max_energized = 0;
    for entry_point in entry_points {
        let mut grid: Vec<Vec<Vec<char>>> = Vec::new();
        for _ in 0..n_rows {
            let mut row: Vec<Vec<char>> = Vec::new();
            for _ in 0..n_cols {
                row.push(Vec::new());
            }
            grid.push(row);
        }
        let mut light_sources: Vec<LightSource> = Vec::new();
        if entry_point.direction == '>' {
            if contraption[entry_point.y][entry_point.x] == '.' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('>');
            }
            if contraption[entry_point.y][entry_point.x] == '-' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('>');
            }
            if contraption[entry_point.y][entry_point.x] == '|' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('^');
                grid[entry_point.y][entry_point.x].push('v');
            }
            if contraption[entry_point.y][entry_point.x] == '/' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                grid[entry_point.y][entry_point.x].push('^');
            }
            if contraption[entry_point.y][entry_point.x] == '\\' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('v');
            }
        } else if entry_point.direction == '<' {
            if contraption[entry_point.y][entry_point.x] == '.' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                grid[entry_point.y][entry_point.x].push('<');
            }
            if contraption[entry_point.y][entry_point.x] == '-' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                grid[entry_point.y][entry_point.x].push('<');
            }
            if contraption[entry_point.y][entry_point.x] == '|' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('^');
                grid[entry_point.y][entry_point.x].push('v');
            }
            if contraption[entry_point.y][entry_point.x] == '/' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('v');
            }
            if contraption[entry_point.y][entry_point.x] == '\\' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                grid[entry_point.y][entry_point.x].push('^');
            }
        } else if entry_point.direction == '^' {
            if contraption[entry_point.y][entry_point.x] == '.' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                grid[entry_point.y][entry_point.x].push('^');
            }
            if contraption[entry_point.y][entry_point.x] == '-' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('<');
                grid[entry_point.y][entry_point.x].push('>');
            }
            if contraption[entry_point.y][entry_point.x] == '|' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '^',
                });
                grid[entry_point.y][entry_point.x].push('^');
            }
            if contraption[entry_point.y][entry_point.x] == '/' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('>');
            }
            if contraption[entry_point.y][entry_point.x] == '\\' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                grid[entry_point.y][entry_point.x].push('<');
            }
        } else if entry_point.direction == 'v' {
            if contraption[entry_point.y][entry_point.x] == '.' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('v');
            }
            if contraption[entry_point.y][entry_point.x] == '-' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('<');
                grid[entry_point.y][entry_point.x].push('>');
            } else if contraption[entry_point.y][entry_point.x] == '|' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: 'v',
                });
                grid[entry_point.y][entry_point.x].push('v');
            } else if contraption[entry_point.y][entry_point.x] == '/' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '<',
                });
                grid[entry_point.y][entry_point.x].push('<');
            } else if contraption[entry_point.y][entry_point.x] == '\\' {
                light_sources.push(LightSource {
                    y: entry_point.y,
                    x: entry_point.x,
                    direction: '>',
                });
                grid[entry_point.y][entry_point.x].push('>');
            }
        }
        while light_sources.len() > 0 {
            let current_light_source = light_sources.pop().unwrap();
            let next_light_sources = get_next_lightsources(&current_light_source, &contraption);
            for next_light_source in next_light_sources {
                let mut is_in_grid = false;
                for light_source in &grid[next_light_source.y][next_light_source.x] {
                    if light_source == &next_light_source.direction {
                        is_in_grid = true;
                        break;
                    }
                }
                if !is_in_grid {
                    grid[next_light_source.y][next_light_source.x]
                        .push(next_light_source.direction);
                    light_sources.push(next_light_source);
                }
            }
        }
        let mut n_energized = 0;
        for row in grid {
            for cell in row {
                if cell.len() > 0 {
                    n_energized += 1;
                }
            }
        }
        max_energized = n_energized.max(max_energized);
    }
    max_energized
}

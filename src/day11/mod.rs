use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};
use std::cmp::Ordering;

enum Op {
    Add,
    Mult,
}

enum Operand {
    Old,
    Num(i64),
}

struct Monkey {
    n_inspections: i64,
    items: Vec<i64>,
    op: Op,
    operand: Operand,
    divisor: i64,
    m_true: u32,
    m_false: u32,
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.n_inspections.cmp(&other.n_inspections)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.n_inspections == other.n_inspections
    }
}

impl Eq for Monkey {}

fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    let lines = read_lines(filename);

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut block: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(text) = line {
            if text.len() > 0 {
                block.push(String::from(text.trim()));
            }

            if block.len() == 6 {
                // process block
                let items: Vec<i64> = block[1]
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|x| x.trim().parse::<i64>().unwrap())
                    .collect();

                let op_comps: Vec<&str> = block[2].split("=").last().unwrap().trim().split_whitespace().collect();
                let divisor = block[3].split_whitespace().last().unwrap().parse::<i64>().unwrap();
                let t = block[4].split_whitespace().last().unwrap().parse::<u32>().unwrap();
                let f = block[5].split_whitespace().last().unwrap().parse::<u32>().unwrap();

                monkeys.push(Monkey{
                    n_inspections: 0,
                    items,
                    op: if op_comps[1] == "*" { Op::Mult } else { Op::Add },
                    operand: if op_comps[2] == "old" { Operand::Old } else { Operand::Num(op_comps[2].parse::<i64>().unwrap()) },
                    divisor,
                    m_true: t,
                    m_false: f,
                });
                block = Vec::new();
            }
        }
    }

    monkeys
}

fn part1() {
    let mut monkeys = parse_monkeys("./src/day11/input11.txt");

    for (i, m) in monkeys.iter().enumerate() {
        println!("{i}, {}, {:?}", m.n_inspections, m.items);
    }

    for round in 0..20 {
        for i in 0..monkeys.len() {

            if monkeys[i].items.len() == 0 { continue; }

            let mut items: Vec<i64> = Vec::new();
            for item in monkeys[i].items.iter() {
                let new_val: i64;

                match monkeys[i].op {
                    Op::Add => {
                        match monkeys[i].operand {
                            Operand::Old => {
                                new_val = item + item;
                            },
                            Operand::Num(x) => {
                                new_val = item + x;
                            }
                        }
                    },
                    Op::Mult => {
                        match monkeys[i].operand {
                            Operand::Old => {
                                new_val = item * item;
                            },
                            Operand::Num(x) => {
                                new_val = item * x;
                            }
                        }
                    }
                }

                items.push(new_val / 3); // LESSON: DO NOT FUCKING CAST TO FLOAT FOR DIVISION
            }

            monkeys[i].n_inspections += items.len() as i64;

            let t = monkeys[i].m_true;
            let f = monkeys[i].m_false;

            for item in items.iter() {
                if item % monkeys[i].divisor == 0 {
                    monkeys[t as usize].items.push(item.clone());
                } else {
                    monkeys[f as usize].items.push(item.clone());
                }
            }

            monkeys[i].items = Vec::new();
        }
        println!("### End of Round {}", round+1);
        for (i, m) in monkeys.iter().enumerate() {
            println!("monkey {i}, {:?}", m.items);
        }
    }

    println!("Results...");
    for (i, m) in monkeys.iter().enumerate() {
        println!("{i}, {}, {:?}", m.n_inspections, m.items);
    }
}

fn part2() {
    let mut monkeys = parse_monkeys("./src/day11/input11.txt");

    let mut divisor_product = 1;
    for m in monkeys.iter() {
        divisor_product *= m.divisor;
    }

    for (i, m) in monkeys.iter().enumerate() {
        println!("{i}, {}, {:?}", m.n_inspections, m.items);
    }

    for round in 0..10000 {
        for i in 0..monkeys.len() {

            if monkeys[i].items.len() == 0 { continue; }

            let mut items: Vec<i64> = Vec::new();
            for item in monkeys[i].items.iter() {
                let new_val: i64;

                match monkeys[i].op {
                    Op::Add => {
                        match monkeys[i].operand {
                            Operand::Old => {
                                new_val = item + item;
                            },
                            Operand::Num(x) => {
                                new_val = item + x;
                            }
                        }
                    },
                    Op::Mult => {
                        match monkeys[i].operand {
                            Operand::Old => {
                                new_val = item * item;
                            },
                            Operand::Num(x) => {
                                new_val = item * x;
                            }
                        }
                    }
                }

                items.push(new_val % divisor_product);
            }

            monkeys[i].n_inspections += items.len() as i64;

            let t = monkeys[i].m_true;
            let f = monkeys[i].m_false;

            for item in items.iter() {
                if item % monkeys[i].divisor == 0 {
                    monkeys[t as usize].items.push(item.clone());
                } else {
                    monkeys[f as usize].items.push(item.clone());
                }
            }

            monkeys[i].items = Vec::new();
        }
        println!("### End of Round {}", round+1);
        for (i, m) in monkeys.iter().enumerate() {
            println!("monkey {i}, {:?}", m.items);
        }
    }

    println!("Results...");

    monkeys.sort();

    for (i, m) in monkeys.iter().enumerate() {
        println!("{i}, {}, {:?}", m.n_inspections, m.items);
    }
}

pub fn main() {
    println!("\n### Day 11 ###");
    part1();
    part2();
}

fn read_lines(filepath: &str) -> Lines<BufReader<fs::File>> {
    let p = Path::new(&filepath);

    match fs::File::open(p) {
        Ok(f) => BufReader::new(f).lines(),
        Err(e) => panic!("Failed to read file {}: {}", p.display(), e)
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn ingest_drawing(drawing_lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>>;

    let last_line = drawing_lines.last()
        .unwrap_or(&String::from(""))
        .clone()
        .to_owned();

    let n_stacks = last_line.split_whitespace()
        .collect::<Vec<&str>>()
        .len();

    stacks = vec![vec![]; n_stacks as usize];

    for i in (0..(drawing_lines.len()-1)).rev() {
        let mut ctr = 0;

        for (j, c) in drawing_lines[i].chars().enumerate() {
            if j == 0 { continue; }

            if (j - 1) % 4 == 0 {
                if c != ' ' {
                    stacks[ctr].push(c);
                }
                ctr += 1;
            }
        }
    }

    return stacks;
}

fn part1() {
    let mut drawing_lines: Vec<String> = vec![];
    let mut stacks: Vec<Vec<char>> = vec![vec![]];

    let mut have_read_drawing = false;

    if let Ok(lines) = read_lines("./src/day5/input5.txt") {
        for line in lines {
            if let Ok(text) = line {

                if !have_read_drawing {
                    if text.len() == 0 {
                        stacks = ingest_drawing(&drawing_lines);
                        have_read_drawing = true;
                        continue;
                    }

                    drawing_lines.push(text.clone());
                    continue;
                }

                let comps: Vec<&str> = text.split(' ').collect();
                if comps.len() != 6 {
                    println!("bad line len!");
                    continue;
                }

                let num_move = comps[1].parse::<u8>().unwrap();
                let from = comps[3].parse::<u8>().unwrap();
                let to = comps[5].parse::<u8>().unwrap();

                let mut tmp: Vec<char> = vec![];

                for _ in 0..num_move {
                    tmp.push(stacks[(from - 1) as usize].pop().unwrap());
                }

                for i in 0..num_move {
                    stacks[(to - 1) as usize].push(tmp[i as usize]);
                }
            }
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

fn part2() {
    let mut drawing_lines: Vec<String> = vec![];
    let mut stacks: Vec<Vec<char>> = vec![vec![]];

    let mut have_read_drawing = false;

    if let Ok(lines) = read_lines("./src/day5/input5.txt") {
        for line in lines {
            if let Ok(text) = line {

                if !have_read_drawing {
                    if text.len() == 0 {
                        stacks = ingest_drawing(&drawing_lines);
                        have_read_drawing = true;
                        continue;
                    }

                    drawing_lines.push(text.clone());
                    continue;
                }

                let comps: Vec<&str> = text.split(' ').collect();
                if comps.len() != 6 {
                    println!("bad line len!");
                    continue;
                }

                let num_move = comps[1].parse::<u8>().unwrap();
                let from = comps[3].parse::<u8>().unwrap();
                let to = comps[5].parse::<u8>().unwrap();

                let mut tmp: Vec<char> = vec![];

                for _ in 0..num_move {
                    tmp.push(stacks[(from - 1) as usize].pop().unwrap());
                }

                for i in (0..num_move).rev() {
                    stacks[(to - 1) as usize].push(tmp[i as usize]);
                }
            }
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

pub fn main() {
    println!("\n### Day 5 ###");
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

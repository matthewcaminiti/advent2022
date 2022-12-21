use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

fn part1() {
    let lines = read_lines("./src/day10/input10.txt");

    let mut checkpoint_idx = 0;

    let mut curr_cycle = 1;
    let mut register = 1;

    let mut tot = 0;

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();

            if curr_cycle == 20 + (40 * checkpoint_idx) {
                tot += register * (20 + (40 * checkpoint_idx));
                checkpoint_idx += 1;
            }

            match comps[0] {
                "noop" => curr_cycle += 1,
                "addx" => {
                    if curr_cycle + 2 > 20 + (40 * checkpoint_idx) {
                        // overshoots checkpoint, no-add
                        tot += register * (20 + (40 * checkpoint_idx));
                        checkpoint_idx += 1;
                    }
                    register += comps[1].parse::<i32>().unwrap();
                    curr_cycle += 2;
                },
                _ => println!("invalid instruction: {}", comps[0])
            }
        }
    }

    println!("Tot: {tot}");
}

fn draw_to_crt(cycle: i32, sprite_pos: i32, crt: &mut [[char; 40]; 6]) {
    let cycle = cycle - 1;
    let row = (cycle as f32 / 40f32).floor() as usize;

    if row >= 6 {
        return;
    }

    if sprite_pos < -1 || sprite_pos > 40 {
        return;
    }

    let x: usize = (cycle % 40) as usize;
    crt[row][x] = if sprite_pos - 1 <= x as i32 && sprite_pos + 1 >= x as i32 { '#' } else { ' ' };
}

fn part2() {
    let lines = read_lines("./src/day10/input10.txt");

    let mut curr_cycle = 1;
    let mut register = 1;

    let mut crt: [[char; 40]; 6] = [['.'; 40]; 6];

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();

            match comps[0] {
                "noop" => {
                    draw_to_crt(curr_cycle, register, &mut crt);
                    curr_cycle += 1
                },
                "addx" => {
                    draw_to_crt(curr_cycle, register, &mut crt);
                    draw_to_crt(curr_cycle + 1, register, &mut crt);

                    register += comps[1].parse::<i32>().unwrap();
                    curr_cycle += 2;
                },
                _ => println!("invalid instruction: {}", comps[0])
            }
        }
    }

    for row in crt.iter() {
        let mut s = String::new();
        for c in row.iter() {
            s.push(*c);
        }
        println!("{s}");
    }
}

pub fn main() {
    println!("\n### Day 10 ###");
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

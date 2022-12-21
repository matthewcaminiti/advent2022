use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Copy)]
#[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn catchup(h_pos: &Pos, t_pos: &Pos) -> Pos {
    let mut dx = h_pos.x - t_pos.x;
    let mut dy = h_pos.y - t_pos.y;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        // t no move, H is adjacent or on top
        return Pos{x: t_pos.x, y: t_pos.y};
    }

    // clamp delta x
    if dx.abs() > 0 {
        dx = if dx < 0 { -1 } else { 1 };
    }

    // clamp delta y
    if dy.abs() > 0 {
        dy = if dy < 0 { -1 } else { 1 };
    }

    return Pos{x: t_pos.x + dx, y: t_pos.y + dy};
}

fn part1() {
    let lines = read_lines("./src/day9/input9.txt");

    let mut t_coords: HashMap<(i32, i32), u32> = HashMap::new();

    let mut h_pos = Pos{x: 0, y: 0};
    let mut t_pos = Pos{x: 0, y: 0};

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();
            if comps.len() != 2 {
                println!("Invalid instruction line: {text}");
                continue;
            }

            let dist = comps[1].parse::<u32>().unwrap();

            for _ in 0..dist {
                match comps[0] {
                    "R" => { h_pos.x += 1; },
                    "L" => { h_pos.x -= 1; },
                    "U" => { h_pos.y += 1; },
                    "D" => { h_pos.y -= 1; },
                    _ => {
                        println!("Invalid instruction supplied: {}", comps[0]);
                        break;
                    },
                }

                let new_t_pos = catchup(&h_pos, &t_pos);
                t_coords.entry((new_t_pos.x, new_t_pos.y)).and_modify(|e| *e += 1).or_insert(1);

                if new_t_pos.x != t_pos.x || new_t_pos.y != t_pos.y {
                    // t moved
                    t_pos = new_t_pos;
                }
            }
        }
    }

    println!("Num coords: {}", t_coords.keys().len());
}

fn part2() {
    let lines = read_lines("./src/day9/input9.txt");

    let mut t_coords: HashMap<(i32, i32), u32> = HashMap::new();

    const NUM_KNOTS: usize = 10;
    let mut knots: [Pos; NUM_KNOTS] = [Pos{x: 0, y: 0}; NUM_KNOTS];

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();
            if comps.len() != 2 {
                println!("Invalid instruction line: {text}");
                continue;
            }

            let dist = comps[1].parse::<u32>().unwrap();

            for _ in 0..dist {
                match comps[0] {
                    "R" => { knots[0].x += 1; },
                    "L" => { knots[0].x -= 1; },
                    "U" => { knots[0].y += 1; },
                    "D" => { knots[0].y -= 1; },
                    _ => {
                        println!("Invalid instruction supplied: {}", comps[0]);
                        break;
                    },
                }

                for i in 0..NUM_KNOTS-1 {
                    let new_pos = catchup(&knots[i], &knots[i + 1]);
                    if i == NUM_KNOTS - 2 {
                        t_coords.entry((new_pos.x, new_pos.y)).and_modify(|e| *e += 1).or_insert(1);
                    }

                    if new_pos.x != knots[i + 1].x || new_pos.y != knots[i + 1].y {
                        // t moved
                        knots[i + 1] = new_pos;
                    }
                }
            }
        }
    }

    println!("Num coords: {}", t_coords.keys().len());
}

pub fn main() {
    println!("\n### Day 9 ###");
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

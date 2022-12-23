use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Debug)]
struct Tile {
    val: u32,
    visited: bool,
    d: u32,
}

struct Pos(usize, usize);

fn parse_map(filename: &str) -> (Vec<Vec<Tile>>, Pos, Pos) {
    let lines = read_lines(filename);

    let mut map: Vec<Vec<Tile>> = Vec::new();

    let mut s_pos = Pos(0, 0);
    let mut e_pos = Pos(0, 0);

    for line in lines {
        if let Ok(text) = line {
            let mut tmp: Vec<Tile> = Vec::new();

            for c in text.chars() {
                let norm_c = match c {
                    'S' => {
                        s_pos = Pos(tmp.len(), map.len());
                        'a'
                    },
                    'E' => {
                        e_pos = Pos(tmp.len(), map.len());
                        'z'
                    },
                    _ => c,
                };

                tmp.push(Tile{
                    val: norm_c as u32,
                    visited: false,
                    d: 0,
                });
            }

            map.push(tmp);
        }
    }

    (map, s_pos, e_pos)
}

fn get_min_dist(map: &mut Vec<Vec<Tile>>, s: &Pos, e: &Pos) -> u32 {
    let mut to_visit: Vec<(usize, usize)> = vec![(s.0, s.1)];

    let mut d = 0;

    map[s.1][s.0].d = d;
    map[s.1][s.0].visited = true;

    while to_visit.len() > 0 {
        let mut next_visit: Vec<(usize, usize)> = Vec::new();

        for pos in to_visit.iter() {
            let x = pos.0;
            let y = pos.1;

            // up
            if y > 0 && map[y - 1][x].val <= map[y][x].val + 1 {
                if !map[y - 1][x].visited || (map[y - 1][x].visited && map[y - 1][x].d > map[y][x].d + 1) {
                    map[y - 1][x].d = d + 1;
                    map[y - 1][x].visited = true;
                    next_visit.push((x, y - 1));
                }
            }
            // down
            if y < map.len() - 1 && map[y + 1][x].val <= map[y][x].val + 1 {
                if !map[y + 1][x].visited || (map[y + 1][x].visited && map[y + 1][x].d > map[y][x].d + 1) {
                    map[y + 1][x].d = d + 1;
                    map[y + 1][x].visited = true;
                    next_visit.push((x, y + 1));
                }
            }
            // right
            if x < map[0].len() - 1 && map[y][x + 1].val <= map[y][x].val + 1 {
                if !map[y][x + 1].visited || (map[y][x + 1].visited && map[y][x + 1].d > map[y][x].d + 1) {
                    map[y][x + 1].d = d + 1;
                    map[y][x + 1].visited = true;
                    next_visit.push((x + 1, y));
                }
            }
            // right
            if x > 0 && map[y][x - 1].val <= map[y][x].val + 1 {
                if !map[y][x - 1].visited || (map[y][x - 1].visited && map[y][x - 1].d > map[y][x].d + 1) {
                    map[y][x - 1].d = d + 1;
                    map[y][x - 1].visited = true;
                    next_visit.push((x - 1, y));
                }
            }
        }

        to_visit = next_visit;
        d += 1;
    }

    let min_dist = map[e.1][e.0].d;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x].d = 0;
            map[y][x].visited = false;
        }
    }

    min_dist
}

fn part1() {
    let ret = parse_map("./src/day12/input12.txt");

    let mut map: Vec<Vec<Tile>> = ret.0;
    let s_pos = ret.1;
    let e_pos = ret.2;

    println!("{}", get_min_dist(&mut map, &s_pos, &e_pos));
}

fn part2() {
    let ret = parse_map("./src/day12/input12.txt");

    let mut map: Vec<Vec<Tile>> = ret.0;
    let e_pos = ret.2;

    let mut starting_positions: Vec<Pos> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].val == 'a' as u32 {
                starting_positions.push(Pos(x, y));
            }
        }
    }

    let mut min_dists: Vec<u32> = Vec::new();
    for sp in starting_positions.iter() {
        min_dists.push(get_min_dist(&mut map, sp, &e_pos));
    }

    min_dists.sort();
    println!("{:?}", min_dists.iter().filter(|x| **x > 0).nth(0).unwrap());
}

pub fn main() {
    println!("\n### Day 12 ###");
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

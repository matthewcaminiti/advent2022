use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord{ x, y }
    }

    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Pair {
    sensor: Coord,
    beacon: Coord,
    radius: i32,
}

impl Pair {
    fn new(sensor: Coord, beacon: Coord) -> Self {
        Pair{ sensor, beacon, radius: (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()}
    }
}

fn get_pairs(filename: &str) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    let lines = read_lines(filename);

    for line in lines {
        if let Ok(text) = line {
            let mut jah = String::new();

            for c in text.chars() {
                if (c as u32 <= '9' as u32 && c as u32 >= '0' as u32) || c == ',' || c == ':' || c == '-' {
                    jah.push(c);
                }
            }

            let nums: Vec<i32> = jah.split(':')
                .flat_map(|x: &str| x.split(',')
                    .map(|y: &str| y.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>())
                .collect();

            pairs.push(Pair::new(
                Coord::new(*nums.get(0).unwrap(), *nums.get(1).unwrap()),
                Coord::new(*nums.get(2).unwrap(), *nums.get(3).unwrap())
            ));
        }
    }

    pairs
}

fn part1() {
    let pairs = get_pairs("./src/day15/input15.txt");

    let mut max_x = 0;
    let mut min_x = 999999999;

    for p in pairs.iter() { 
        let fawad = p.sensor.x + p.radius;
        if fawad > max_x {
            max_x = fawad;
        }
        
        let backwad = p.sensor.x - p.radius;
        if backwad < min_x {
            min_x = backwad;
        }
    }

    let mut ctr = 0;

    let y = 2_000_000;

    for x in min_x..=max_x {
        let coord = Coord{x, y};

        for p in pairs.iter() {
            if (coord.x == p.beacon.x && coord.y == p.beacon.y) ||
                (coord.x == p.sensor.x && coord.y == p.sensor.y) {
                break;
            }

            if coord.dist(&p.sensor) <= p.radius {
                ctr += 1;
                break;
            }
        }
    }

    println!("Num pos that cannot contain beacon at y={y}: {ctr}");
}

fn part2() {
    let pairs = get_pairs("./src/day15/input15.txt");

    const SEARCH_LOWER_BOUND: i32 = 0;
    const SEARCH_UPPER_BOUND: i32 = 4_000_000;

    for p in pairs.iter() {
        // get bordering coords
        let mut border_coords: Vec<Coord> = Vec::new();

        for inc in 1..=1 {
            let extended_radius = p.radius + inc;

            let mut d_x = extended_radius * -1;
            let mut d_y = extended_radius - d_x.abs();

            while p.sensor.x + d_x < SEARCH_LOWER_BOUND {
                d_x += 1;
            }

            while d_x <= extended_radius && p.sensor.x + d_x <= SEARCH_UPPER_BOUND {
                if p.sensor.x + d_x < SEARCH_LOWER_BOUND {
                    d_x += 1;
                    d_y = extended_radius - d_x.abs();
                    continue;
                }
                // get coord
                if p.sensor.y + d_y >= SEARCH_LOWER_BOUND && p.sensor.y + d_y <= SEARCH_UPPER_BOUND {
                    border_coords.push(Coord::new(p.sensor.x + d_x, p.sensor.y + d_y));
                }
                if d_y > 0 {
                    if p.sensor.y - d_y >= SEARCH_LOWER_BOUND && p.sensor.y - d_y <= SEARCH_UPPER_BOUND {
                        border_coords.push(Coord::new(p.sensor.x + d_x, p.sensor.y - d_y));
                    }
                }

                d_x += 1;
                d_y = extended_radius - d_x.abs();
            }
        }

        println!("{} num border coords", border_coords.len());

        // check each coord if covered against all pairs
        for coord in border_coords.iter() {
            let mut covered = false;

            for p in pairs.iter() {
                if (coord.x == p.beacon.x && coord.y == p.beacon.y) ||
                    (coord.x == p.sensor.x && coord.y == p.sensor.y) {
                    break;
                }

                if coord.dist(&p.sensor) <= p.radius {
                    covered = true;
                    break;
                }
            }

            if !covered {
                println!("UNCOVERED COORD FOUND! {coord}, freq: {}", (coord.x as i64 * 4_000_000i64) + coord.y as i64);
                return
            }
        }
    }
}

pub fn main() {
    println!("\n### Day 15 ###");
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

use std::{fs, fmt};
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

struct Coord(i32, i32);

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn parse_paths() -> (Vec<Vec<Coord>>, Coord, Coord) {
    let lines = read_lines("./src/day14/input14.txt");

    let mut paths = Vec::new();
    let mut x_dim = Coord(1000, 0);
    let mut y_dim = Coord(1000, 0);

    for line in lines {
        if let Ok(text) = line {
            let coords: Vec<Coord> = text.split(" -> ")
                .map(|x| x.trim().split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect())
                .map(|x: Vec<i32>| Coord(*x.get(0).unwrap(), *x.get(1).unwrap()))
                .collect();

            for c in coords.iter() {
                if c.0 < x_dim.0 {
                    x_dim.0 = c.0;
                } else if c.0 >= x_dim.1 {
                    x_dim.1 = c.0;
                }

                if c.1 < y_dim.0 {
                    y_dim.0 = c.1;
                } else if c.1 >= y_dim.1 {
                    y_dim.1 = c.1;
                }
            }

            paths.push(coords);
        }
    }

    (paths, x_dim, y_dim)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for y in map.iter() {
        for x in y.iter() {
            print!("{x}");
        }
        println!("");
    }
}

fn drop_sand(map: &mut Vec<Vec<char>>, x_offset: i32) -> bool {
    let mut sand = Coord(500 - x_offset, 0);

    loop {
        if map[sand.1 as usize][sand.0 as usize] != '.' {
            // maxed!
            return false;
        }

        if sand.1 + 1 >= map.len() as i32 {
            // into void
            return false
        }

        if map[sand.1 as usize + 1][sand.0 as usize] == '.' {
            // can move down
            sand.1 += 1;
            continue;
        }

        if sand.0 - 1 < 0 {
            // into void
            return false
        }

        if map[sand.1 as usize + 1][sand.0 as usize - 1] == '.' {
            // down and left
            sand.1 += 1;
            sand.0 -= 1;
            continue;
        }

        if sand.0 + 1 >= map.first().unwrap().len() as i32 {
            // into void
            return false
        }

        if map[sand.1 as usize + 1][sand.0 as usize + 1] == '.' {
            // down and right
            sand.1 += 1;
            sand.0 += 1;
            continue;
        }

        // blocked!
        map[sand.1 as usize][sand.0 as usize] = 'o';

        return true;
    }
}

fn part1() {
    let (paths, x_dim, y_dim) = parse_paths();

    let x_offset = x_dim.0;

    let mut map: Vec<Vec<char>> = Vec::new();

    for _ in 0..=y_dim.1 {
        let mut row = Vec::new();

        for _ in x_dim.0..=x_dim.1 {
            row.push('.');
        }

        map.push(row);
    }

    for p in paths.iter() {
        let mut prev_coord = p.first().unwrap();

        for coord in p[1..].iter() {
            // draw from prev_coord to coord
            if coord.0 == prev_coord.0 {
                // vertical draw
                for (i, y) in map.iter_mut().enumerate() {
                    if (i as i32 <= coord.1 && i as i32 >= prev_coord.1) ||
                       (i as i32 <= prev_coord.1 && i as i32 >= coord.1) {
                        y[coord.0 as usize - x_offset as usize] = '#';
                    }
                }
            } else if coord.1 == prev_coord.1 {
                // horizontal draw
                for i in 0..map.first().unwrap().len() {
                    if (i as i32 <= coord.0 - x_offset && i as i32 >= prev_coord.0 - x_offset) ||
                       (i as i32 <= prev_coord.0 - x_offset && i as i32 >= coord.0 - x_offset) {
                        map.get_mut(coord.1 as usize).unwrap()[i] = '#';
                    }
                }
            } else {
                panic!("Diagonal draw attempted");
            }
            // then update prev_coord
            prev_coord = coord;
        }
    }

    let mut ctr = 0;
    loop {
        // drop sand
        if !drop_sand(&mut map, x_offset) {
            // into abyss
            break;
        }
        ctr += 1;
    }

    println!("{ctr} num sands came to rest");
}

fn part2() {
    let (paths, x_dim, y_dim) = parse_paths();

    let x_offset = x_dim.0 - (y_dim.1 * 2);

    let mut map: Vec<Vec<char>> = Vec::new();

    for _ in 0..=y_dim.1 {
        let mut row = Vec::new();

        for _ in 0..=y_dim.1 * 4 {
            row.push('.');
        }

        map.push(row);
    }

    {
        let mut row = Vec::new();

        for _ in 0..=y_dim.1 * 4 {
            row.push('.');
        }

        map.push(row)
    }

    {
        let mut row = Vec::new();

        for _ in 0..=y_dim.1 * 4 {
            row.push('#');
        }

        map.push(row)
    }

    for p in paths.iter() {
        let mut prev_coord = p.first().unwrap();

        for coord in p[1..].iter() {
            // draw from prev_coord to coord
            if coord.0 == prev_coord.0 {
                // vertical draw
                for (i, y) in map.iter_mut().enumerate() {
                    if (i as i32 <= coord.1 && i as i32 >= prev_coord.1) ||
                       (i as i32 <= prev_coord.1 && i as i32 >= coord.1) {
                        y[coord.0 as usize - x_offset as usize] = '#';
                    }
                }
            } else if coord.1 == prev_coord.1 {
                // horizontal draw
                for i in 0..map.first().unwrap().len() {
                    if (i as i32 <= coord.0 - x_offset && i as i32 >= prev_coord.0 - x_offset) ||
                       (i as i32 <= prev_coord.0 - x_offset && i as i32 >= coord.0 - x_offset) {
                        map.get_mut(coord.1 as usize).unwrap()[i] = '#';
                    }
                }
            } else {
                panic!("Diagonal draw attempted");
            }
            // then update prev_coord
            prev_coord = coord;
        }
    }

    let mut ctr = 0;
    loop {
        // drop sand
        if !drop_sand(&mut map, x_offset) {
            // into abyss
            break;
        }

        ctr += 1;
    }

    println!("{ctr} num sands came to rest");
}

pub fn main() {
    println!("\n### Day 14 ###");
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

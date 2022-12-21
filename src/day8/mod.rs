use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Debug)]
struct Tree {
    height: u8,
    visited: bool,
    visible: bool,
}

fn is_visible(x: usize, y: usize, forest: &Vec<Vec<Tree>>) -> bool {
    if x == 0 || y == 0 || x == forest[y].len() - 1 || y == forest.len() - 1 {
        return true;
    }

    let w = forest[y].len();
    let h = forest.len();

    let curr = &forest[y][x];

    let mut right_covered = false;
    let mut left_covered = false;
    let mut up_covered = false;
    let mut down_covered = false;

    // right
    for i in (x+1)..w {
        if forest[y][i].height >= curr.height {
            right_covered = true;
            break;
        }
    }

    if !right_covered { return true; }

    // left
    for i in (0..x).rev() {
        if forest[y][i].height >= curr.height {
            left_covered = true;
            break;
        }
    }

    if !left_covered { return true; }

    // up
    for i in (0..y).rev() {
        if forest[i][x].height >= curr.height {
            up_covered = true;
            break;
        }
    }

    if !up_covered { return true; }

    // down
    for i in (y+1)..h {
        if forest[i][x].height >= curr.height {
            down_covered = true;
            break;
        }
    }

    if !down_covered { return true; }

    return false;
}

fn num_visible(x: usize, y: usize, forest: &Vec<Vec<Tree>>) -> u32 {
    if x == 0 || y == 0 || x == forest[y].len() - 1 || y == forest.len() - 1 {
        return 0;
    }

    let w = forest[y].len();
    let h = forest.len();

    let curr = &forest[y][x];

    let mut n_right = 0u32;
    let mut n_left = 0u32;
    let mut n_up = 0u32;
    let mut n_down = 0u32;

    // right
    for i in (x+1)..w {
        n_right += 1;
        if forest[y][i].height >= curr.height {
            break;
        }
    }

    // left
    for i in (0..x).rev() {
        n_left += 1;
        if forest[y][i].height >= curr.height {
            break;
        }
    }

    // up
    for i in (0..y).rev() {
        n_up += 1;
        if forest[i][x].height >= curr.height {
            break;
        }
    }

    // down
    for i in (y+1)..h {
        n_down += 1;
        if forest[i][x].height >= curr.height {
            break;
        }
    }

    return n_right * n_left * n_up * n_down;
}

fn part1() {
    let lines = read_lines("./src/day8/input8.txt");

    let mut forest: Vec<Vec<Tree>> = vec![];

    for line in lines {
        let mut row: Vec<Tree> = vec![];

        if let Ok(text) = line {
            for c in text.chars() {
                row.push(Tree{
                    height: c.to_digit(10).unwrap() as u8,
                    visited: false,
                    visible: false,
                });
            }
        }

        forest.push(row);
    }

    let mut num_visible = 0;

    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            if is_visible(x, y, &forest) {
                num_visible += 1;
            }
        }
    }

    println!("Num visible: {num_visible}");
}

fn part2() {
    let lines = read_lines("./src/day8/input8.txt");

    let mut forest: Vec<Vec<Tree>> = vec![];

    for line in lines {
        let mut row: Vec<Tree> = vec![];

        if let Ok(text) = line {
            for c in text.chars() {
                row.push(Tree{
                    height: c.to_digit(10).unwrap() as u8,
                    visited: false,
                    visible: false,
                });
            }
        }

        forest.push(row);
    }

    let mut max_scenic_score = 0;

    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            let score = num_visible(x, y, &forest);
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    println!("Max score: {max_scenic_score}");
}

pub fn main() {
    println!("\n### Day 8 ###");
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

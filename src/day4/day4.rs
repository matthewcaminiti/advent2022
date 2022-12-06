use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() {
    let mut count = 0;

    if let Ok(lines) = read_lines("./src/day4/input4.txt") {
        for line in lines {
            if let Ok(text) = line {
                let ranges: Vec<&str> = text.split(',').collect();

                if ranges.len() != 2 {
                    println!("Not two ranges on line: {} found", ranges.len());
                    continue;
                }

                let range1: Vec<i32> = ranges[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
                let range2: Vec<i32> = ranges[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

                if range1.len() != 2 && range2.len() != 2 {
                    println!("Bad range lengths: {}, {}", range1.len(), range2.len());
                    continue;
                }

                if range1[0] < range2[0] {
                    if range2[1] <= range1[1] {
                        count += 1;
                    }
                } else if range2[0] < range1[0] {
                    if range1[1] <= range2[1] {
                        count += 1;
                    }
                } else {
                    count += 1;
                }
            }
        }
    }

    println!("Count of full overlaps: {}", count);
}

fn part2() {
    let mut count = 0;

    if let Ok(lines) = read_lines("./src/day4/input4.txt") {
        for line in lines {
            if let Ok(text) = line {
                let ranges: Vec<&str> = text.split(',').collect();

                if ranges.len() != 2 {
                    println!("Not two ranges on line: {} found", ranges.len());
                    continue;
                }

                let range1: Vec<i32> = ranges[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
                let range2: Vec<i32> = ranges[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

                if range1.len() != 2 && range2.len() != 2 {
                    println!("Bad range lengths: {}, {}", range1.len(), range2.len());
                    continue;
                }

                if (range1[0] >= range2[0] && range1[0] <= range2[1]) ||
                    (range1[1] >= range2[0] && range1[1] <= range2[1]) ||
                    (range2[0] >= range1[0] && range2[0] <= range1[1]) ||
                    (range2[1] >= range1[0] && range2[1] <= range1[1]) {
                    count += 1;
                }
            }
        }
    }
    println!("Count of any overlaps: {}", count);
}

pub fn main() {
    println!("### Day 4 ###");
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

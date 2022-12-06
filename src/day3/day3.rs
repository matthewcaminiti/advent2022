use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn part1() {
    let mut first_container = HashMap::new();
    let mut priority_tot = 0;

    if let Ok(lines) = read_lines("./src/day3/input3.txt") {
        for line in lines {
            if let Ok(text) = line {

                for (i, c) in text.chars().enumerate() {
                    if i < text.len() / 2 {
                        first_container.entry(c).or_insert(1);
                    } else {
                        if first_container.contains_key(&c) {
                            let ascii = c as u32;
                            if ascii >= 65 && ascii <= 90 {
                                priority_tot += ascii - 65 + 27;
                            } else {
                                priority_tot += ascii - 97 + 1;
                            }
                            break;
                        }
                    }
                }

                first_container = HashMap::new();
            }
        }
    }

    println!("Total: {}", priority_tot);
}

pub fn part2() {
    let mut first_container = HashMap::new();
    let mut second_container = HashMap::new();
    let mut priority_tot = 0;
    let mut count = 1;

    if let Ok(lines) = read_lines("./src/day3/input3.txt") {
        for line in lines {
            if let Ok(text) = line {

                if count % 3 == 0 {
                    // is at the end of group, check existences
                    for c in text.chars() {
                        if first_container.contains_key(&c) && second_container.contains_key(&c) {
                            let ascii = c as u32;
                            if ascii >= 65 && ascii <= 90 {
                                priority_tot += ascii - 65 + 27;
                            } else {
                                priority_tot += ascii - 97 + 1;
                            }
                            break;
                        }
                    }

                    first_container = HashMap::new();
                    second_container = HashMap::new();
                } else if (count - 1) % 3 == 0 {
                    for c in text.chars() {
                        first_container.entry(c).or_insert(1);
                    }
                } else if (count - 2) % 3 == 0 {
                    for c in text.chars() {
                        second_container.entry(c).or_insert(1);
                    }
                }

                count += 1;
            }
        }
    }

    println!("Total: {}", priority_tot);
}

pub fn main() {
    println!("### Day 3 ###");
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

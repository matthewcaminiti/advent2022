use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() {
    let mut max_cal = 0;
    let mut curr_cal = 0;

    if let Ok(lines) = read_lines("./src/day1/input1.txt") {
        for line in lines {
            if let Ok(text) = line {

                if text.len() == 0 {
                    if curr_cal > max_cal {
                        max_cal = curr_cal;
                    }

                    curr_cal = 0;
                } else {
                    if let Ok(num) = text.parse::<i32>() {
                        curr_cal += num;
                    }
                }
            }
        }
    }

    println!("Max cal: {}", max_cal);
}

fn part2() {
    const NUM_TOP_DAWGZ: usize = 3;
    let mut top_dawgz: [i32; NUM_TOP_DAWGZ] = [0; NUM_TOP_DAWGZ];
    let mut smallest_i = 0;
    let mut curr_cal = 0;

    if let Ok(lines) = read_lines("./src/day1/input1.txt") {
        for line in lines {
            if let Ok(text) = line {
                if text.len() == 0 {
                    let mut bigger = false;
                    for i in 0..NUM_TOP_DAWGZ {
                        if curr_cal > top_dawgz[i] {
                            bigger = true;
                            break;
                        }
                    }

                    if bigger {
                        top_dawgz[smallest_i] = curr_cal;

                        let mut smallest = 23232323;
                        for i in 0..NUM_TOP_DAWGZ {
                            if top_dawgz[i] < smallest {
                                smallest_i = i;
                                smallest = top_dawgz[i];
                            }
                        }
                    }

                    curr_cal = 0;
                } else {
                    if let Ok(num) = text.parse::<i32>() {
                        curr_cal += num;
                    }
                }
            }
        }
    }

    let mut tot = 0;
    for i in 0..NUM_TOP_DAWGZ {
        tot += top_dawgz[i];
    }

    println!("Sum of top {} dawgz: {}", NUM_TOP_DAWGZ, tot);
}

pub fn main() {
    println!("### Day 1 ###");
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// A -> Rock -> X -> 1
// B -> Paper -> Y -> 2
// C -> Scissors -> Z -> 3
// 0 loss
// 3 draw
// 6 win

pub fn part1() {
    let mut tot = 0;

    if let Ok(lines) = read_lines("./src/day2/input2.txt") {
        for line in lines {
            if let Ok(text) = line {

                let v: Vec<&str> = text.split(" ").collect();
                if v.len() != 2 { continue; }

                match v[0] {
                    "A" => {
                        match v[1] {
                            "X" => tot += 4,
                            "Y" => tot += 8,
                            "Z" => tot += 3,
                            _ => println!("AHHHHH")
                        }
                    },
                    "B" => {
                        match v[1] {
                            "X" => tot += 1,
                            "Y" => tot += 5,
                            "Z" => tot += 9,
                            _ => println!("AHHHHH")
                        }
                    },
                    "C" => {
                        match v[1] {
                            "X" => tot += 7,
                            "Y" => tot += 2,
                            "Z" => tot += 6,
                            _ => println!("AHHHHH")
                        }
                    },
                    _ => println!("AHHHHH")
                }
            }
        }
    }

    println!("Your total: {}", tot);
}

fn part2() {
    let mut tot = 0;

    if let Ok(lines) = read_lines("./src/day2/input2.txt") {
        for line in lines {
            if let Ok(text) = line {
                let v: Vec<&str> = text.split(" ").collect();
                if v.len() != 2 { continue; }

                match v[1] {
                    "X" => {
                        // lose
                        match v[0] {
                            "A" => tot += 3,
                            "B" => tot += 1,
                            "C" => tot += 2,
                            _ => continue,
                        }
                    },
                    "Y" => {
                        // draw
                        tot += 3;
                        match v[0] {
                            "A" => tot += 1,
                            "B" => tot += 2,
                            "C" => tot += 3,
                            _ => continue,
                        }
                    },
                    "Z" => {
                        // win
                        tot += 6;
                        match v[0] {
                            "A" => tot += 2,
                            "B" => tot += 3,
                            "C" => tot += 1,
                            _ => continue,
                        }
                    },
                    _ => continue,
                };

            }
        }
    }

    println!("Total with second strat: {}", tot);
}

pub fn main() {
    println!("\n### Day 2 ###");
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

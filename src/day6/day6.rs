use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn do_your_thing_21(num_distinct_chars: usize) -> i32 {
    let text = read_text("./src/day6/input6.txt");

    let mut acc: HashMap<char, i32> = HashMap::new();
    let mut q: VecDeque<char> = VecDeque::new();

    for (i, c) in text.chars().enumerate() {
        if q.len() < num_distinct_chars - 1 {
            q.push_back(c);
            acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
            continue;
        }

        if (acc.len() < num_distinct_chars - 1) || acc.contains_key(&c) {
            let pop = q.pop_front().unwrap();

            if acc.get(&pop).unwrap() == &1 {
                acc.remove(&pop);
            } else {
                acc.entry(pop).and_modify(|e| *e -= 1);
            }

            acc.entry(c).and_modify(|e| *e += 1).or_insert(1);

            q.push_back(c);
            continue;
        }

        return (i + 1) as i32;
    }

    return 0;
}

fn part1() {
    let jah = do_your_thing_21(4);
    println!("Found unique at: {}", jah);
}

fn part2() {
    let jah = do_your_thing_21(14);
    println!("Found unique at: {}", jah);
}

pub fn main() {
    println!("\n### Day 6 ###");
    part1();
    part2();
}

fn read_text(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Err(err) => panic!("couldn't read {}: {}", display, err),
        Ok(_) => buf,
    }
}

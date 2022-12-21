use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Debug)]
struct Link {
    parent_id: u32,
    child_id: u32,
}

struct Dir {
    name: String,
    size: u32,
}

fn part1() {
    let lines = read_lines("./src/day7/input7.txt");

    let mut skeleton: Vec<Link> = vec![];
    let mut dirs: HashMap<u32, Dir> = HashMap::new();

    let mut dir_id_ctr = 1u32;

    let mut curr_dir_id = 0u32;

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();

            if comps.len() < 2 {
                println!("Invalid line: {text}");
                continue;
            }

            if comps[0] == "$" {
                // command
                match comps[1] {
                    "cd" => {
                        if comps.len() != 3 {
                            println!("Invalid num elements of command: {comps:?}");
                            continue;
                        }

                        if curr_dir_id == 0 {
                            // first dir
                            curr_dir_id = dir_id_ctr;
                            dirs.insert(dir_id_ctr, Dir{name: String::from(comps[2]), size: 0});
                            dir_id_ctr += 1;
                        } else {
                            // changing from dir
                            if comps[2] == ".." {
                                match skeleton.iter().find(|link| link.child_id == curr_dir_id) {
                                    Some(link) => curr_dir_id = link.parent_id,
                                    None => panic!("Tried to `cd ..` without a parent!")
                                }
                            } else {
                                let mut found = (false, 0u32);
                                for link in skeleton.iter() {
                                    if link.parent_id == curr_dir_id {
                                        match dirs.get(&link.child_id) {
                                            Some(d) if d.name == comps[2] => found = (true, link.child_id),
                                            _ => {},
                                        }
                                        if found.0 { break; }
                                    }
                                }

                                if found.0 {
                                    curr_dir_id = found.1;
                                } else {
                                    // new directory
                                    skeleton.push(Link{parent_id: curr_dir_id, child_id: dir_id_ctr});
                                    curr_dir_id = dir_id_ctr;
                                    dirs.insert(dir_id_ctr, Dir{name: String::from(comps[2]), size: 0});
                                    dir_id_ctr += 1;
                                }
                            }
                        }
                    },
                    "ls" => {},
                    _ => println!("Invalid command found: {}", comps[1]),
                }
            } else {
                // file
                if let Some(_) = dirs.get_mut(&curr_dir_id) {
                    let filesize = comps[0].parse::<u32>().unwrap_or(0);
                    let mut next_dirs: Vec<u32> = Vec::new();
                    let mut curr_dirs: Vec<u32> = vec![curr_dir_id];
                    while curr_dirs.len() > 0 {
                        for dir_id in curr_dirs.iter() {
                            if let Some(d) = dirs.get_mut(dir_id) {
                                d.size += filesize;
                            }
                            for l in skeleton.iter() {
                                if &l.child_id == dir_id {
                                    next_dirs.push(l.parent_id);
                                }
                            }
                        }

                        curr_dirs = next_dirs.clone();
                        next_dirs = Vec::new();
                    }
                } else {
                    println!("Unable to find dir with id: {curr_dir_id}")
                }
            }
        }
    }

    if skeleton.len() == 0 {
        println!("No skeleton to traverse!");
        return;
    }

    let mut tot = 0;
    for (_, dir) in dirs.iter() {
        if dir.size <= 100000 {
            tot += dir.size;
        }
    }

    println!("Tot: {tot}");
}

fn part2() {
    let lines = read_lines("./src/day7/input7.txt");

    let mut skeleton: Vec<Link> = vec![];
    let mut dirs: HashMap<u32, Dir> = HashMap::new();

    let mut dir_id_ctr = 1u32;

    let mut curr_dir_id = 0u32;

    for line in lines {
        if let Ok(text) = line {
            let comps: Vec<&str> = text.split_whitespace().collect();

            if comps.len() < 2 {
                println!("Invalid line: {text}");
                continue;
            }

            if comps[0] == "$" {
                // command
                match comps[1] {
                    "cd" => {
                        if comps.len() != 3 {
                            println!("Invalid num elements of command: {comps:?}");
                            continue;
                        }

                        if curr_dir_id == 0 {
                            // first dir
                            curr_dir_id = dir_id_ctr;
                            dirs.insert(dir_id_ctr, Dir{name: String::from(comps[2]), size: 0});
                            dir_id_ctr += 1;
                        } else {
                            // changing from dir
                            if comps[2] == ".." {
                                match skeleton.iter().find(|link| link.child_id == curr_dir_id) {
                                    Some(link) => curr_dir_id = link.parent_id,
                                    None => panic!("Tried to `cd ..` without a parent!")
                                }
                            } else {
                                let mut found = (false, 0u32);
                                for link in skeleton.iter() {
                                    if link.parent_id == curr_dir_id {
                                        match dirs.get(&link.child_id) {
                                            Some(d) if d.name == comps[2] => found = (true, link.child_id),
                                            _ => {},
                                        }
                                        if found.0 { break; }
                                    }
                                }

                                if found.0 {
                                    curr_dir_id = found.1;
                                } else {
                                    // new directory
                                    skeleton.push(Link{parent_id: curr_dir_id, child_id: dir_id_ctr});
                                    curr_dir_id = dir_id_ctr;
                                    dirs.insert(dir_id_ctr, Dir{name: String::from(comps[2]), size: 0});
                                    dir_id_ctr += 1;
                                }
                            }
                        }
                    },
                    "ls" => {},
                    _ => println!("Invalid command found: {}", comps[1]),
                }
            } else {
                // file
                if let Some(_) = dirs.get_mut(&curr_dir_id) {
                    let filesize = comps[0].parse::<u32>().unwrap_or(0);

                    let mut next_dirs: Vec<u32> = Vec::new();
                    let mut curr_dirs: Vec<u32> = vec![curr_dir_id];

                    while curr_dirs.len() > 0 {
                        for dir_id in curr_dirs.iter() {
                            if let Some(d) = dirs.get_mut(dir_id) {
                                d.size += filesize;
                            }
                            for l in skeleton.iter() {
                                if &l.child_id == dir_id {
                                    next_dirs.push(l.parent_id);
                                }
                            }
                        }

                        curr_dirs = next_dirs.clone();
                        next_dirs = Vec::new();
                    }
                } else {
                    println!("Unable to find dir with id: {curr_dir_id}")
                }
            }
        }
    }

    if skeleton.len() == 0 {
        println!("No skeleton to traverse!");
        return;
    }

    let tot_storage = 70000000;
    let required_storage = 30000000;
    let used_storage = dirs.get(&1u32).unwrap().size;
    let unused = tot_storage - used_storage;

    let mut min_delete = required_storage;
    for (_, dir) in dirs.iter() {
        if unused + dir.size >= required_storage {
            if dir.size < min_delete {
                min_delete = dir.size;
            }
        }
    }

    println!("Min delete: {min_delete}");
}

pub fn main() {
    println!("\n### Day 7 ###");
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

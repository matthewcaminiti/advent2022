use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};

#[derive(Clone)]
struct Tunnel {
    a: String,
    b: String,
    dist: i32,
}

impl Tunnel {
    fn new(a: String, b: String, dist: i32) -> Self {
        Tunnel{a, b, dist}
    }
}

impl Display for Tunnel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.dist)
    }
}

fn shortest_path(source: &String, tunnels: &Vec<Tunnel>, valves: &HashMap<String, i32>) -> HashMap<String, i32> {
    // Dijkstra's
    let mut dist: HashMap<String, i32> = HashMap::new();
    let mut prev: HashMap<&String, String> = HashMap::new();
    let mut q: Vec<&String> = Vec::new();
    
    let inf = 1_000_000_000;

    for (k, _) in valves.iter() {
        dist.insert(k.clone(), inf);
        prev.insert(k, String::from(""));
        q.push(k);
    }

    dist.entry(source.clone()).and_modify(|e| *e = 0);

    while q.len() > 0 {
        let mut min_dist = inf;
        let mut min_i = 0;

        for (i, vertex) in q.iter().enumerate() {
            let v_dist = dist.get(*vertex).unwrap();

            if *v_dist < min_dist {
                min_dist = *v_dist;
                min_i = i;
            }
        }

        let u = q.remove(min_i);

        let mut neighbors: Vec<(&String, &i32)> = Vec::new();
        for t in tunnels.iter() {
            if &t.a == u || &t.b == u {
                if &t.a == u && q.contains(&&t.b) {
                    neighbors.push((&t.b, &t.dist));
                } else if &t.b == u && q.contains(&&t.a) {
                    neighbors.push((&t.a, &t.dist));
                }
            }
        }

        for neighbor in neighbors.iter() {
            let alt = dist.get(u).unwrap() + neighbor.1;
            if alt < *dist.get(neighbor.0).unwrap() {
                dist.entry(neighbor.0.clone()).and_modify(|e| *e = alt);
                prev.entry(neighbor.0).and_modify(|e| *e = u.clone());
            }
        }
    }

    dist
}

fn get_tunnels_and_valves(filename: &str) -> (Vec<Tunnel>, HashMap<String, i32>) {
    let lines = read_lines(filename);

    let mut tunnels: Vec<Tunnel> = Vec::new();

    let mut valves: HashMap<String, i32> = HashMap::new();

    for line in lines {
        if let Ok(text) = line {
            let jah: Vec<&str> = text.split(' ').collect();

            let curr = String::from(jah.get(1).unwrap().to_owned());
            let rate = jah.get(4).unwrap().split('=').last().unwrap().trim_end_matches(';').parse::<i32>().unwrap();

            valves.insert(curr.clone(), rate);

            let leads: Vec<String> = jah[9..].iter().map(|x| String::from(x.trim_end_matches(',').to_owned())).collect();
            for lead in leads.iter() {
                let mut exists = false;
                for t in tunnels.iter() {
                    if (t.a == curr && &t.b == lead) || (t.b == curr && &t.a == lead) {
                        exists = true;
                    }
                }

                if !exists {
                    tunnels.push(Tunnel::new(curr.clone(), lead.clone(), 1));
                }
            }
        }
    }

    (tunnels, valves)
}

fn get_max_option(
    curr: &String,
    time: i32,
    min_dists: &HashMap<String, HashMap<String, i32>>,
    valves: &HashMap<String, i32>,
    opened_valves: &HashMap<String, bool>
) -> i32 {
    if time > 30 || opened_valves.len() == valves.len() {
        return 0;
    }

    let rate = valves.get(curr).unwrap();

    let mut max_path = 0;

    let mut next_opened_valves = opened_valves.clone();

    next_opened_valves.insert(curr.clone(), true);

    for (k, dist) in min_dists.get(curr).unwrap().iter() {
        if k == "AA" || k == curr || opened_valves.contains_key(k) { continue; }

        let mut new_time = time + dist;
        if curr != "AA" {
            // time incr for opening valve
            new_time += 1;
        }

        let tmp = ((30 - (time + 1)) * *rate) + get_max_option(k, new_time, min_dists, valves, &next_opened_valves);

        if tmp > max_path {
            max_path = tmp;
        }
    }

    if opened_valves.len() == valves.len() - 1 {
        let tmp = (30 - (time + 1)) * *rate;
        if tmp > max_path {
            max_path = tmp;
        }
    }

    max_path
}

fn part1() {
    let (mut tunnels, mut valves) = get_tunnels_and_valves("./src/day16/input.txt");

    let mut dead_valves: Vec<String> = Vec::new();

    for (valve, rate) in valves.iter() {
        if *rate == 0 && valve != "AA" {
            // node to be removed
            let mut orphans: Vec<(String, i32)> = Vec::new();

            let mut i = 0;
            while i < tunnels.len() {
                if &tunnels[i].a == valve {
                    // remove me
                    let removed = tunnels.remove(i);
                    orphans.push((removed.b, removed.dist));
                } else if &tunnels[i].b == valve {
                    // remove me
                    let removed = tunnels.remove(i);
                    orphans.push((removed.a, removed.dist));
                } else {
                    i += 1;
                }
            }

            if orphans.len() < 2 {
                continue;
            }

            let base = orphans.get(0).unwrap();
            for i in 1..orphans.len() {
                tunnels.push(Tunnel::new(base.0.clone(), orphans.get(i).unwrap().0.clone(), base.1 + orphans.get(i).unwrap().1));
            }

            dead_valves.push(valve.clone());
        }
    }

    for v in dead_valves.iter() {
        valves.remove(v);
    }

    let mut min_dists: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for (k, _) in valves.iter() {
        min_dists.insert(k.clone(), shortest_path(k, &tunnels, &valves));
    }

    // starting at AA
    // time < 30 as cap for total
    // consider every option at every valve (including opening, if rate > 0)

    let opened_valves: HashMap<String, bool> = HashMap::new();
    let start = String::from("AA");
    println!("max pressure release: {}", get_max_option(&start, 0, &min_dists, &valves, &opened_valves));

}

pub fn main() {
    println!("\n### Day 16 ###");
    part1();
}

fn read_lines(filepath: &str) -> Lines<BufReader<fs::File>> {
    let p = Path::new(&filepath);

    match fs::File::open(p) {
        Ok(f) => BufReader::new(f).lines(),
        Err(e) => panic!("Failed to read file {}: {}", p.display(), e)
    }
}

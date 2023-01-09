use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead, Lines};
use std::fmt;

#[derive(Debug, Clone)]
enum Word {
    Num(i32),
    Arr(Vec<Word>)
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match &self {
            Word::Num(x) => { write!(f, "{}", x) },
            Word::Arr(x) => {
                write!(f, "[")?;
                for (i, w) in x.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }

                    write!(f, "{}", w)?;
                }
                write!(f,"]")
            },
        }

    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_words(self, other) {
            WordCmp::Valid => Ordering::Less,
            WordCmp::Invalid => Ordering::Greater,
            WordCmp::Skip => Ordering::Equal
        }
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        match compare_words(self, other) {
            WordCmp::Skip => true,
            _ => false
        }
    }
}

impl Eq for Word {}

fn parse_word(s: String) -> Vec<Word> {
    let mut jah: Vec<Word> = Vec::new();
    
    if s.len() == 0 {
        return jah;
    }

    let mut tmp = String::new();

    let mut ctr = 0;
    for c in s[1..s.len()-1].chars() {
        match c {
            '[' => {
                ctr += 1;
                tmp.push(c);
            },
            ']' => {
                ctr -= 1;
                tmp.push(c);
                if ctr == 0 {
                    jah.push(Word::Arr(parse_word(String::from(tmp))));
                    tmp = String::new();
                }
            }
            ',' => {
                if ctr == 0 && tmp.len() > 0 {
                    jah.push(Word::Num(tmp.parse::<i32>().unwrap()));
                    tmp = String::new();
                } else if tmp.len() > 0 {
                    tmp.push(c);
                }
            },
            _ => tmp.push(c)
        }
    }

    if tmp.len() > 0 {
        jah.push(Word::Num(tmp.parse::<i32>().unwrap()));
    }

    jah
}

fn parse_all_pairs(filepath: &str) -> Vec<(Vec<Word>, Vec<Word>)> {
    let lines = read_lines(filepath);

    let mut pairs: Vec<(Vec<Word>, Vec<Word>)> = Vec::new();

    let mut c1: Vec<Word> = Vec::new();
    let mut c2: Vec<Word> = Vec::new();

    let mut jah = true;

    for line in lines {
        if let Ok(text) = line {
            // println!("{text}");
            if text.len() == 0 {
                pairs.push((c1, c2));
                c1 = Vec::new();
                c2 = Vec::new();
                continue;
            }

            if jah {
                c1 = parse_word(text);
                jah = !jah;

                // print!("[");
                // for (i, w) in c1.iter().enumerate() {
                //     if i > 0 { print!(",") }
                //     print!("{w}");
                // }
                // print!("]");
                // println!("");
            } else {
                c2 = parse_word(text);
                jah = !jah;

                // print!("[");
                // for (i, w) in c2.iter().enumerate() {
                //     if i > 0 { print!(",") }
                //     print!("{w}");
                // }
                // print!("]");
                // println!("");
            }
        }
    }
    
    pairs.push((c1, c2));

    pairs
}

enum WordCmp {
    Valid,
    Invalid,
    Skip
}

fn compare_words(w1: &Word, w2: &Word) -> WordCmp {
    match w1 {
        Word::Num(x) => {
            match w2 {
                Word::Num(y) => {
                    if x > y {
                        return WordCmp::Invalid
                    } else if x == y {
                        return WordCmp::Skip
                    } else {
                        return WordCmp::Valid
                    }
                },
                Word::Arr(_) => {
                    // convert lhs to arr and compare
                    return compare_words(&Word::Arr(vec![Word::Num(*x)]), w2);
                }
            }
        },
        Word::Arr(x) => {
            match w2 {
                Word::Num(y) => {
                    // convert rhs to arr and compare
                    return compare_words(w1, &Word::Arr(vec![Word::Num(*y)]));
                },
                Word::Arr(y) => {
                    let mut li = 0;
                    let mut ri = 0;

                    while li < x.len() && ri < y.len() {
                        match compare_words(&x[li], &y[ri]) {
                            WordCmp::Valid => return WordCmp::Valid,
                            WordCmp::Invalid => return WordCmp::Invalid,
                            WordCmp::Skip => {} 
                        }

                        li += 1;
                        ri += 1;
                    }

                    if y.len() < x.len() {
                        return WordCmp::Invalid;
                    } else if x.len() < y.len() {
                        return WordCmp::Valid;
                    } else {
                        return WordCmp::Skip;
                    }
                }
            }
        }
    }
}

fn part1() {
    let pairs = parse_all_pairs("./src/day13/input13.txt");

    let mut valid_sum = 0;

    // get indices of pairs that are in the RIGHT order (indexed by 1)
    // calculate sum of indices
    for (i, p) in pairs.iter().enumerate() {
        let l = Word::Arr((&p.0).clone());
        let r = Word::Arr((&p.1).clone());

        match compare_words(&l, &r) {
            WordCmp::Valid => valid_sum += i + 1,
            WordCmp::Invalid => {},
            WordCmp::Skip => {},
        }
    }

    println!("Valid indices sum: {valid_sum}");
}

fn part2() {
    let pairs = parse_all_pairs("./src/day13/input13.txt");

    let mut flattened: Vec<Word> = Vec::new();

    for p in pairs {
        flattened.push(Word::Arr((&p.0).clone()));
        flattened.push(Word::Arr((&p.1).clone()));
    }

    flattened.push(Word::Arr(vec![Word::Arr(vec![Word::Num(2)])]));
    flattened.push(Word::Arr(vec![Word::Arr(vec![Word::Num(6)])]));

    flattened.sort();

    let mut i1 = 0;
    let mut i2 = 0;

    for (i, jah) in flattened.iter().enumerate() {
        match jah {
            Word::Arr(x) => {
                if x.len() == 1 {
                    match x.first().unwrap() {
                        Word::Arr(y) => {
                            if y.len() == 1 {
                                match y.first().unwrap() {
                                    Word::Num(2) => i1 = i + 1,
                                    Word::Num(6) => i2 = i + 1,
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    println!("Prod of indices: {i1} x {i2} = {}", i1 * i2);
}

pub fn main() {
    println!("\n### Day 13 ###");
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

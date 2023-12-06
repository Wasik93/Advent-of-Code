use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Triplet {
    from: u64,
    to: u64,
    range: u64,
}

struct Doublet {
    from: u64,
    range: u64,
}

fn part1(lines: &Vec<String>) {
    let mut f = false;
    let mut id = 0;
    let mut xdd = 1;
    let mut tmp = false;
    let mut seeds: Vec<u64> = Vec::new();
    let mut db: Vec<String> = Vec::new();
    db.push("soil".to_string());
    db.push("fertilizer".to_string());
    db.push("water".to_string());
    db.push("light".to_string());
    db.push("temperature".to_string());
    db.push("humidity".to_string());
    db.push("location".to_string());

    let mut phases: Vec<Vec<Triplet>> = Vec::new();
    for i in 0..7 {
        let tmp_vec: Vec<Triplet> = Vec::new();
        phases.push(tmp_vec);
    }
    for line in lines {
        let split_line = line.clone();
        let parts: Vec<_> = split_line.split_whitespace().collect();
        if !f {
            f = true;
            for i in 1..parts.len() {
                seeds.push(parts[i].parse().unwrap());
            }
            continue;
        }
        if parts.is_empty() {
            tmp = true;
            id += 1;
            id = id - xdd;
            xdd = 0;
            continue;
        }
        if tmp {
            tmp = false;
            continue;
        }
        let tmp = Triplet { from: parts[1].parse().unwrap(), to: parts[0].parse().unwrap(), range: parts[2].parse().unwrap() };
        phases[id].push(tmp);
    }
    for i in 0..7 {
        phases[i].sort_by(|a, b| a.from.cmp(&b.from));
    }
    let mut min_val = 418461546212222u64;
    for i in 0..seeds.len() {
//println!("seed - {}", seeds[i]);
        for j in 0..phases.len() {
            for k in 0..phases[j].len() {
                if seeds[i] >= phases[j][k].from && (seeds[i] < phases[j][k].from + phases[j][k].range) {
                    seeds[i] = seeds[i] - phases[j][k].from + phases[j][k].to;
                    break;
                }
            }
//println!("{} - {}", db[j], seeds[i]);
        }
        if min_val > seeds[i] {
            min_val = seeds[i];
        }
    }
    println!("{}", min_val);
}


fn part2(lines: &Vec<String>) {
    let mut f = false;
    let mut id = 0;
    let mut xdd = 1;
    let mut tmp = false;
    let mut seeds: Vec<Doublet> = Vec::new();
    let mut db: Vec<String> = Vec::new();
    db.push("soil".to_string());
    db.push("fertilizer".to_string());
    db.push("water".to_string());
    db.push("light".to_string());
    db.push("temperature".to_string());
    db.push("humidity".to_string());
    db.push("location".to_string());

    let mut phases: Vec<Vec<Triplet>> = Vec::new();
    for i in 0..7 {
        let tmp_vec: Vec<Triplet> = Vec::new();
        phases.push(tmp_vec);
    }
    for line in lines {
        let split_line = line.clone();
        let parts: Vec<_> = split_line.split_whitespace().collect();
        if !f {
            f = true;
            let mut prev = 0;
            for i in 1..parts.len() {
                if i % 2 == 0 {
                    let mut tmp_doublet = Doublet { from: 0, range: 0 };
                    tmp_doublet.from = prev;
                    tmp_doublet.range = parts[i].parse().unwrap();
                    seeds.push(tmp_doublet);
                } else {
                    prev = parts[i].parse().unwrap();
                }
            }
            continue;
        }
        if parts.is_empty() {
            tmp = true;
            id += 1;
            id = id - xdd;
            xdd = 0;
            continue;
        }
        if tmp {
            tmp = false;
            continue;
        }
        let tmp = Triplet { from: parts[1].parse().unwrap(), to: parts[0].parse().unwrap(), range: parts[2].parse().unwrap() };
        phases[id].push(tmp);
    }
    for i in 0..7 {
        phases[i].sort_by(|a, b| a.from.cmp(&b.from));
    }

    for i in 0..88151870 { //value from part1 + 1
        let mut xdd = i;
        for tmp_j in 0..7 {
            let j = 7 - tmp_j - 1;
            for k in 0..phases[j].len() {
                if xdd >= phases[j][k].to && (xdd < phases[j][k].to + phases[j][k].range) {
                    xdd = xdd - phases[j][k].to + phases[j][k].from;
                    break;
                }
            }
        }
        for j in 0..seeds.len() {
            if xdd >= seeds[j].from && xdd < seeds[j].from + seeds[j].range {
                println!("{}", i);
                return;
            }
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let day = "DAY5";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

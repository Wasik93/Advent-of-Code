use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::env::current_exe;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use num::integer::lcm;

#[derive(Clone)]
struct Node {
    left: usize,
    right: usize,
}

fn parse_string(s: &str) -> usize {
    let mut score = 0;
    for i in s.chars() {
        if i.is_alphabetic() {
            score = score * 26 + i as usize - 'A' as usize;
        }
    }
    return score;
}

fn part1(lines: &Vec<String>) {
    let mut instructions: Vec<_> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    nodes.resize(17576, Node { left: 0, right: 0 });
    let mut id = 0;
    for line in lines {
        let mut split_line = line.clone();
        if id == 0 {
            instructions = split_line.chars().collect();
            id += 1;
            continue;
        }
        if id == 1 {
            id += 1;
            continue;
        }
        id += 1;
        let mut parts: Vec<_> = split_line.split_whitespace().collect();
        let mut n = parse_string(parts[0]);
        let l = parse_string(parts[2]);
        let r = parse_string(parts[3]);
        nodes[n].left = l;
        nodes[n].right = r;
    }
    let mut current_node = 0;
    let mut index_i = 0;
    let mut n = instructions.len();
    let mut counter = 0;
    while current_node != 17575 {
        if index_i >= n {
            index_i = 0;
        }
        if instructions[index_i] == 'L' {
            current_node = nodes[current_node].left;
        } else {
            current_node = nodes[current_node].right;
        }
        index_i += 1;
        counter += 1;
    }
    println!("{}", counter);
}


fn part2(lines: &Vec<String>) {
    let mut instructions: Vec<_> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    let mut current_node: Vec<usize> = Vec::new();
    nodes.resize(17576, Node { left: 0, right: 0 });
    let mut id = 0;
    for line in lines {
        let mut split_line = line.clone();
        if id == 0 {
            instructions = split_line.chars().collect();
            id += 1;
            continue;
        }
        if id == 1 {
            id += 1;
            continue;
        }
        id += 1;
        let mut parts: Vec<_> = split_line.split_whitespace().collect();
        let mut n = parse_string(parts[0]);
        let l = parse_string(parts[2]);
        let r = parse_string(parts[3]);
        nodes[n].left = l;
        nodes[n].right = r;
        if n % 26 == 0 {
            current_node.push(n);
        }
    }
    let mut index_i = 0;
    let mut n = instructions.len();
    let mut counter = 0;
    let mut f = false;
    let mut cycles: Vec<usize> = Vec::new();
    cycles.resize(current_node.len(), 0);
    loop {
        if index_i >= n {
            index_i = 0;
        }
        for i in 0..current_node.len() {
            if instructions[index_i] == 'L' {
                current_node[i] = nodes[current_node[i]].left;
            } else {
                current_node[i] = nodes[current_node[i]].right;
            }
        }
        index_i += 1;
        counter += 1;
        f = true;
        for i in 0..current_node.len() {
            if current_node[i] % 26 == 25 && cycles[i] == 0 {
                cycles[i] = counter;
            }
            if cycles[i] == 0 {
                f = false;
            }
        }
        if f {
            break;
        }
    }
    let mut score = 1;
    for i in 0..cycles.len() {
        score = lcm(score, cycles[i]);
    }
    println!("{}", score);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let day = "DAY8";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

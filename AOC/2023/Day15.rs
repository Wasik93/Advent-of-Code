use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut sum = 0;
    const PRIME:u32 = 17;
    const MOD:u32 = 256;
    for line in lines{
        let mut parts:Vec<_> = line.split_whitespace().collect();
        let mut steps: Vec<_> = parts[0].split(',').collect();
        for i in steps{
            let mut value = 0;
            for j in i.chars() {
                value = value + (j as u32);
                value = value * PRIME;
                value = value % MOD;
            }
            sum += value;
        }
    }
    println!("{}", sum);
}
fn part2(lines: &Vec<String>) {
    let mut sum = 0;
    const PRIME:u32 = 17;
    const MOD:u32 = 256;
    let mut boxes:Vec<Vec<(&str, usize)>> = Vec::new();
    boxes.resize(256, Vec::new());
    for line in lines{
        let mut parts:Vec<_> = line.split_whitespace().collect();
        let mut steps: Vec<_> = parts[0].split(',').collect();
        for i in steps{
            let mut value = 0;
            let mut tmp_vec = i.split(|c| c == '=' || c == '-').collect::<Vec<_>>();
            for j in tmp_vec[0].chars() {
                value = value + (j as u32);
                value = value * PRIME;
                value = value % MOD;
            }
            if tmp_vec[1] == "" {
                let mut id = 0;
                let mut f = false;
                for i in &boxes[value as usize]{
                    if i.0 == tmp_vec[0]{
                        f = true;
                        break;
                    }
                    id += 1;
                }
                if f {
                    while id != boxes[value as usize].len() - 1 {
                        boxes[value as usize][id].0 = boxes[value as usize][id + 1].0;
                        boxes[value as usize][id].1 = boxes[value as usize][id + 1].1;
                        id += 1;
                    }
                    boxes[value as usize].remove(id);
                }
            }
            else{
                let mut f = false;
                for i in 0..boxes[value as usize].len() {
                    if boxes[value as usize][i].0 == tmp_vec[0]{
                        boxes[value as usize][i].1 = tmp_vec[1].parse().unwrap();
                        f = true;
                        break;
                    }
                }
                if !f {
                    boxes[value as usize].push((tmp_vec[0], tmp_vec[1].parse().unwrap()))
                }
            }
        }
    }
    let mut id = 0;
    for i in 0..boxes.len(){
        id += 1;
        let mut id_box = 0;
        for j in &boxes[i]{
            id_box += 1;
            println!("{} {} {}", id, id_box, j.1);
            sum = sum + id * id_box * j.1;
        }
    }
    println!("{}", sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let day = "DAY15";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

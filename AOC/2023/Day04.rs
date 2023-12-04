use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn intersection(winning_numbers: &mut Vec<u32>, my_numbers: &mut Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..winning_numbers.len() {
        for j in 0..my_numbers.len() {
            if winning_numbers[i] == my_numbers[j] {
                sum += 1;
            }
        }
    }
    return sum;
}

fn part1(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let mut split_line = line.clone();
        let mut split_line = &split_line[9..];
        let mut parts: Vec<_> = split_line.split(' ').collect();
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut my_numbers: Vec<u32> = Vec::new();
        let mut f = false;
        for i in 0..parts.len() {
            if parts[i].is_empty() {
                continue;
            }
            if parts[i] == "|" {
                f = true;
                continue;
            }
            if !f {
                winning_numbers.push(parts[i].parse::<u32>().unwrap());
            }
            if f {
                my_numbers.push(parts[i].parse::<u32>().unwrap());
            }
        }
        sum += intersection(&mut winning_numbers, &mut my_numbers);
    }

    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut sum = 0;
    let mut numbers: Vec<u32> = Vec::new();
    for line in lines {
        let mut split_line = line.clone();
        let mut split_line = &split_line[9..];
        let mut parts: Vec<_> = split_line.split(' ').collect();
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut my_numbers: Vec<u32> = Vec::new();
        let mut f = false;
        for i in 0..parts.len() {
            if parts[i].is_empty() {
                continue;
            }
            if parts[i] == "|" {
                f = true;
                continue;
            }
            if !f {
                winning_numbers.push(parts[i].parse::<u32>().unwrap());
            }
            if f {
                my_numbers.push(parts[i].parse::<u32>().unwrap());
            }
        }
        numbers.push(intersection(&mut winning_numbers, &mut my_numbers) + 1);
    }
    let mut copy:Vec<u32> = Vec::new();
    for i in 0..numbers.len(){
        copy.push(1);
    }
    for i in 0..numbers.len() {
        let card:usize = numbers[i] as usize;
        for j in i + 1..min(i + card, numbers.len()) {
            copy[j] = copy[j] + copy[i];
        }
    }
    for i in 0..numbers.len() {
        sum += copy[i];
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
    let day = "DAY4";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}
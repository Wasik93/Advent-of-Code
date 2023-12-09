use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn check_zero(numbers: &Vec<isize>) -> bool {
    for i in 0..numbers.len() {
        if numbers[i] != 0 {
            return false;
        }
    }
    return true;
}

fn calculate_last_number(numbers: &Vec<Vec<isize>>, level: usize) -> isize{
    if level == numbers.len() - 1{
        return 0;
    }
    let n = numbers[level].len() - 1;
    return numbers[level][n] + calculate_last_number(&numbers, level + 1);
}

fn calculate_first_number(numbers: &Vec<Vec<isize>>, level: usize) -> isize{
    if level == numbers.len() - 1 {
        return 0;
    }
    return numbers[level][0] - calculate_first_number(&numbers, level + 1);
}

fn part1(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut numbers: Vec<Vec<isize>> = Vec::new();
        numbers.push(Vec::new());
        for i in 0..parts.len() {
            numbers[0].push(parts[i].parse().unwrap());
        }
        let mut id = 0;
        while !check_zero(&numbers[id]) {
            let mut new_line: Vec<isize> = Vec::new();
            for i in 0..numbers[id].len() - 1{
                new_line.push(numbers[id][i + 1] - numbers[id][i]);
            }
            numbers.push(new_line);
            id += 1;
        }
        sum += calculate_last_number(&numbers, 0);
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut numbers: Vec<Vec<isize>> = Vec::new();
        numbers.push(Vec::new());
        for i in 0..parts.len() {
            numbers[0].push(parts[i].parse().unwrap());
        }
        let mut id = 0;
        while !check_zero(&numbers[id]) {
            let mut new_line: Vec<isize> = Vec::new();
            for i in 0..numbers[id].len() - 1{
                new_line.push(numbers[id][i + 1] - numbers[id][i]);
            }
            numbers.push(new_line);
            id += 1;
        }
        sum += calculate_first_number(&numbers, 0);

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
    let day = "DAY9";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}
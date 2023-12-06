use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut time_line = lines[0].clone();
    let mut dist_line = lines[1].clone();
    let mut time_vec_tmp: Vec<_> = time_line.split_whitespace().collect();
    let mut dist_vec_tmp: Vec<_> = dist_line.split_whitespace().collect();
    let mut time_vec: Vec<usize> = Vec::new();
    let mut dist_vec: Vec<usize> = Vec::new();
    for i in 1..time_vec_tmp.len() {
        time_vec.push(time_vec_tmp[i].parse().unwrap());
        dist_vec.push(dist_vec_tmp[i].parse().unwrap());
    }
    let mut sum = 0;
    for i in 0..time_vec.len() {
        let mut sum_race = 0;
        let mut time = time_vec[i];
        let mut dist = dist_vec[i];
        for j in 1..time {
            let mut score = (time - j) * j;
            if score > dist{
                sum_race += 1;
            }
        }
        if sum == 0{
            sum = sum_race;
        }
        else{
            sum = sum * sum_race;
        }
    }
    println!("{}", sum);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn part2(lines: &Vec<String>) {
    let mut time_line = lines[0].clone();
    remove_whitespace(&mut time_line);
    let mut dist_line = lines[1].clone();
    remove_whitespace(&mut dist_line);
    let mut time_vec_tmp: Vec<_> = time_line.split(':').collect();
    let mut dist_vec_tmp: Vec<_> = dist_line.split(':').collect();
    let mut time_vec: Vec<usize> = Vec::new();
    let mut dist_vec: Vec<usize> = Vec::new();
    for i in 1..time_vec_tmp.len() {
        time_vec.push(time_vec_tmp[i].parse().unwrap());
        dist_vec.push(dist_vec_tmp[i].parse().unwrap());
    }
    let mut sum = 0;
    for i in 0..time_vec.len() {
        let mut sum_race = 0;
        let mut time = time_vec[i];
        let mut dist = dist_vec[i];
        for j in 1..time {
            let mut score = (time - j) * j;
            if score > dist{
                sum_race += 1;
            }
        }
        if sum == 0{
            sum = sum_race;
        }
        else{
            sum = sum * sum_race;
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
    let day = "DAY6";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}
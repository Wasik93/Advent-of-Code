use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Node{
    x: isize,
    y: isize,
}

fn part1(lines: &Vec<String>) {
    let mut galaxies: Vec<Node> = Vec::new();
    let mut universe: Vec<Vec<isize>> = Vec::new();
    let mut id_i = 0;
    for line in lines {
        let mut id_j = 0;
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec: Vec<isize> = Vec::new();
        for i in parts[0].chars() {
            if i == '#' {
                galaxies.push(Node{x: id_i, y:id_j});
                tmp_vec.push(1);
            }
            else {
                tmp_vec.push(0);
            }
            id_j += 1;
        }
        universe.push(tmp_vec);
        id_i += 1;
    }
    let mut rows: Vec<isize> = Vec::new();
    let mut cols: Vec<isize> = Vec::new();
    for i in 0..universe.len() {
        let mut row_sum = 0;
        for j in 0..universe[i].len() {
            row_sum += universe[i][j];
        }
        if row_sum == 0 {
            rows.push(1);
        }
        else {
            rows.push(0);
        }
    }
    for j in 0..universe[0].len() {
        let mut column_sum = 0;
        for i in 0..universe.len() {
            column_sum += universe[i][j];
        }
        if column_sum == 0 {
            cols.push(1);
        }
        else {
            cols.push(0);
        }
    }
    for i in 1..rows.len(){
        rows[i] = rows[i] + rows[i - 1];
    }
    for i in 1..cols.len(){
        cols[i] = cols[i] + cols[i - 1];
    }
    let mut sum_val = 0;
    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len() {
            let val = abs(galaxies[i].x - galaxies[j].x) + abs(galaxies[i].y - galaxies[j].y) + abs(rows[galaxies[j].x as usize] - rows[galaxies[i].x as usize]) + abs(cols[galaxies[i].y as usize] - cols[galaxies[j].y as usize]);
            sum_val += val;
        }
    }
    println!("{}", sum_val);
}

fn abs(p0: isize) -> isize {
    return if p0 < 0 {
        p0 * (-1)
    } else {
        p0
    }
}

fn part2(lines: &Vec<String>) {
    let mut galaxies: Vec<Node> = Vec::new();
    let mut universe: Vec<Vec<isize>> = Vec::new();
    let mut id_i = 0;
    for line in lines {
        let mut id_j = 0;
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec: Vec<isize> = Vec::new();
        for i in parts[0].chars() {
            if i == '#' {
                galaxies.push(Node{x: id_i, y:id_j});
                tmp_vec.push(1);
            }
            else {
                tmp_vec.push(0);
            }
            id_j += 1;
        }
        universe.push(tmp_vec);
        id_i += 1;
    }
    let mut rows: Vec<isize> = Vec::new();
    let mut cols: Vec<isize> = Vec::new();
    for i in 0..universe.len() {
        let mut row_sum = 0;
        for j in 0..universe[i].len() {
            row_sum += universe[i][j];
        }
        if row_sum == 0 {
            rows.push(1);
        }
        else {
            rows.push(0);
        }
    }
    for j in 0..universe[0].len() {
        let mut column_sum = 0;
        for i in 0..universe.len() {
            column_sum += universe[i][j];
        }
        if column_sum == 0 {
            cols.push(1);
        }
        else {
            cols.push(0);
        }
    }
    for i in 1..rows.len(){
        rows[i] = rows[i] + rows[i - 1];
    }
    for i in 1..cols.len(){
        cols[i] = cols[i] + cols[i - 1];
    }
    let mut sum_val = 0;
    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len() {
            let val = abs(galaxies[i].x - galaxies[j].x) + abs(galaxies[i].y - galaxies[j].y) + 999999*abs(rows[galaxies[j].x as usize] - rows[galaxies[i].x as usize]) + 999999*abs(cols[galaxies[i].y as usize] - cols[galaxies[j].y as usize]);
            sum_val += val;
        }
    }
    println!("{}", sum_val);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let day = "DAY11";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

use std::char::MAX;
use std::cmp::{max, min};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::mem::swap;
use std::path::Path;

fn value(s: &str) -> u128 {
    let mut val: u128 = 0;
    for i in s.chars() {
        val = val * 2;
        if i == '#' {
            val = val + 1;
        }
        if i == '.' {
            val = val;
        }
    }
    return val;
}

fn solve_1(val_ver: &Vec<u128>, val_hor: &Vec<u128>) -> u128 {
    let mut sum: u128 = 0;
    for i in 0..val_ver.len() - 1 {
        let mut j: isize = i as isize;
        let mut k = i + 1;
        let mut f = true;
        while j >= 0 && k < val_ver.len() {
            if val_ver[j as usize] == val_ver[k] {
                j -= 1;
                k += 1;
            } else {
                f = false;
                break;
            }
        }
        if f {
            sum = sum + i as u128 + 1;
        }
    }
    for i in 0..val_hor.len() - 1 {
        let mut j: isize = i as isize;
        let mut k = i + 1;
        let mut f = true;
        while j >= 0 && k < val_hor.len() {
            if val_hor[j as usize] == val_hor[k] {
                j -= 1;
                k += 1;
            } else {
                f = false;
                break;
            }
        }
        if f {
            sum = sum + 100 * (i as u128 + 1);
        }
    }
    return sum;
}


fn solve_2(val_ver: &Vec<u128>, val_hor: &Vec<u128>) -> u128 {
    let mut sum: u128 = 0;
    let mut count = 0;
    for i in 0..val_ver.len() - 1 {
        let mut j: isize = i as isize;
        let mut k = i + 1;
        let mut f = true;
        let mut c = false;
        while j >= 0 && k < val_ver.len() {
            if val_ver[j as usize] == val_ver[k] {
                j -= 1;
                k += 1;
            } else if !c {
                if check_lines(val_ver[j as usize], val_ver[k]) {
                    c = true;
                    j -= 1;
                    k += 1;
                } else {
                    f = false;
                    break;
                }
            } else {
                f = false;
                break;
            }
        }
        if f && c {
            sum = sum + i as u128 + 1;
            count += 1;
        }
    }
    for i in 0..val_hor.len() - 1 {
        let mut j: isize = i as isize;
        let mut k = i + 1;
        let mut f = true;
        let mut c = false;
        while j >= 0 && k < val_hor.len() {

            if val_hor[j as usize] == val_hor[k] {
                j -= 1;
                k += 1;
            } else if !c {
                if check_lines(val_hor[j as usize], val_hor[k]) {
                    c = true;
                    j -= 1;
                    k += 1;
                } else {
                    f = false;
                    break;
                }
            } else {
                f = false;
                break;
            }
        }
        if f && c {
            sum = sum + 100 * (i as u128 + 1);
            count += 1;
        }
    }
    return sum;
}

fn check_lines(first: u128, second: u128) -> bool {
    let mut x = max(first, second);
    let mut y = min(first, second);
    let mut p: u128 = 2;
    if x % 2 == 1{
        if x - 1 == y {
            return true;
        }
    }
    while p <= x {
        if x - p == y {
            return true;
        }
        p = p * 2;
    }
    return false;
}

fn part1(lines: &Vec<String>) {
    let mut sum: u128 = 0;
    let mut table: Vec<Vec<char>> = Vec::new();
    let mut values_vertical: Vec<u128> = Vec::new();
    let mut values_horizontal: Vec<u128> = Vec::new();
    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        if parts.is_empty() {
            for j in 0..table[0].len() {
                let mut val = 0;
                for i in 0..table.len() {
                    val = val * 2;
                    if table[i][j] == '#' {
                        val += 1;
                    } else {
                        val += 0;
                    }
                }
                values_vertical.push(val);
            }
            sum += solve_1(&values_vertical, &values_horizontal);
            table.clear();
            values_vertical.clear();
            values_horizontal.clear();
            continue;
        }
        table.push(parts[0].chars().collect());
        values_horizontal.push(value(&parts[0]));
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut sum: u128 = 0;
    let mut table: Vec<Vec<char>> = Vec::new();
    let mut values_vertical: Vec<u128> = Vec::new();
    let mut values_horizontal: Vec<u128> = Vec::new();
    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        if parts.is_empty() {
            for j in 0..table[0].len() {
                let mut val = 0;
                for i in 0..table.len() {
                    val = val * 2;
                    if table[i][j] == '#' {
                        val += 1;
                    } else {
                        val += 0;
                    }
                }
                values_vertical.push(val);
            }
            sum += solve_2(&values_vertical, &values_horizontal);
            table.clear();
            values_vertical.clear();
            values_horizontal.clear();
            continue;
        }
        table.push(parts[0].chars().collect());
        values_horizontal.push(value(&parts[0]));
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
    let day = "DAY13";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

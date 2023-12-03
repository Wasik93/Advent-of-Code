use std::collections::HashSet;
use std::ffi::c_char;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::Map;
use std::path::Path;
use std::u32;

fn check(i: isize, j: isize, array: &mut Vec<Vec<char>>, n: isize, m: isize) {
    for index_i in -1..2 {
        for index_j in -1..2 {
            if i + index_i < 0 || i + index_i >= m || j + index_j < 0 || j + index_j >= n {
                continue;
            }
            if array[(i + index_i) as usize][(j + index_j) as usize].is_numeric() {
                array[(i + index_i) as usize][(j + index_j) as usize] = 'n';
            }
        }
    }
}

fn check2(i: isize, j: isize, array: &mut Vec<Vec<char>>, numbers: &Vec<u32>, vector_id: &Vec<Vec<usize>>, n: usize, m: usize) -> usize {
    let mut tmp = 0;
    let mut f = false;
    for index_i in -1..2 {
        for index_j in -1..2 {
            if i + index_i < 0 || i + index_i >= m as isize || j + index_j < 0 || j + index_j >= n as isize {
                continue;
            }
            if array[(i + index_i) as usize][(j + index_j) as usize].is_numeric() {
                if !f {
                    tmp += 1;
                    f = true;
                }
            } else {
                f = false;
            }
        }
        f = false;
    }
    let mut first = 0;
    let mut second = 0;
    if tmp == 2 {
        tmp = 0;
        f = false;
        for index_i in -1..2 {
            for index_j in -1..2 {
                if i + index_i < 0 || i + index_i >= m as isize || j + index_j < 0 || j + index_j >= n as isize {
                    continue;
                }
                if array[(i + index_i) as usize][(j + index_j) as usize].is_numeric() {
                    if !f {
                        tmp += 1;
                        f = true;
                        if tmp == 1 {
                            first = numbers[vector_id[(i + index_i) as usize][(j + index_j) as usize]];
                        } else {
                            second = numbers[vector_id[(i + index_i) as usize][(j + index_j) as usize]];
                        }
                    }
                } else {
                    f = false;
                }
            }
            f = false;
        }
    }
    return (first * second) as usize;
}

fn part1(lines: &Vec<String>) {
    let mut array: Vec<Vec<char>> = Vec::new();
    let n: isize = lines[0].len() as isize;
    let m: isize = lines.len() as isize;
    for line in lines {
        let mut copy_line = line.clone();
        let mut tmp_vec: Vec<char> = copy_line.chars().collect();
        array.push(tmp_vec.clone());
    }
    for i in 0..n {
        for j in 0..m {
            if array[i as usize][j as usize] != '.' && !array[i as usize][j as usize].is_numeric() {
                array[i as usize][j as usize] = 'x';
            }
        }
    }

    let mut array2 = array.clone();

    for i in 0..m {
        for j in 0..n {
            if array2[i as usize][j as usize] == 'x' {
                check(i, j, &mut array2, n, m);
            }
        }
    }
    let mut sum = 0;
    let mut number = 0;
    let mut f = false;
    for i in 0..m {
        for j in 0..n {
            let i_u: usize = i as usize;
            let j_u: usize = j as usize;
            if array[i_u][j_u].is_numeric() {
                let mut tmp_value = array[i_u][j_u] as u32 - '0' as u32;
                number = number * 10 + tmp_value;
            } else if f {
                sum += number;
                number = 0;
                f = false;
            } else {
                number = 0;
                f = false;
            }
            if array2[i_u][j_u] == 'n' {
                f = true;
            }
        }
        if f {
            sum += number;
            number = 0;
            f = false;
        }
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut array: Vec<Vec<char>> = Vec::new();
    let n: usize = lines[0].len();
    let m: usize = lines.len();
    for line in lines {
        let mut copy_line = line.clone();
        let mut tmp_vec: Vec<char> = copy_line.chars().collect();
        array.push(tmp_vec.clone());
    }
    for i in 0..n {
        for j in 0..m {
            if array[i][j] != '.' && !array[i][j].is_numeric() {
                array[i][j] = 'x';
            }
        }
    }
    let mut numbers: Vec<u32> = Vec::new();
    numbers.push(0);
    let mut vector_id: Vec<Vec<usize>> = Vec::new();
    let mut id = 1;
    let mut f = false;
    let mut number = 0;
    for i in 0..m {
        vector_id.push(Vec::new());
        for j in 0..n {
            vector_id[i].push(0);
        }
    }
    for i in 0..m {
        for j in 0..n {
            if array[i][j].is_numeric() {
                vector_id[i][j] = id;
                number = number * 10 + array[i][j] as u32 - '0' as u32;
                f = true;
            } else {
                if f {
                    numbers.push(number);
                    id += 1;
                }
                f = false;
                number = 0;
            }
        }
        if f {
            numbers.push(number);
            id += 1;
            f = false;
            number = 0;
        }
    }
    let mut sum = 0;
    for i in 0..m {
        for j in 0..n {
            if array[i][j] == 'x' {
                sum += check2(i as isize, j as isize, &mut array, &numbers, &vector_id, n, m);
            }
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
    let day = "DAY3";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

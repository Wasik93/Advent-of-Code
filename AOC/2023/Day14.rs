use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut fields: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec: Vec<char> = Vec::new();
        for i in parts[0].chars() {
            tmp_vec.push(i);
        }
        fields.push(tmp_vec);
    }
    push_north(&mut fields);
    let n = fields.len();
    let mut sum = 0;
    for i in 0..fields.len() {
        let mut val = n - i;
        for j in 0..fields[i].len() {
            if fields[i][j] == 'O' {
                sum += n - i;
            }
        }
    }
    println!("{}", sum);
}

fn push_north(fields: &mut Vec<Vec<char>>) {
    for i in 1..fields.len() {
        for j in 0..fields[i].len() {
            let mut i_up = i;
            while i_up > 0 {
                if fields[i_up][j] == 'O' {
                    if fields[i_up - 1][j] == '.' {
                        fields[i_up - 1][j] = 'O';
                        fields[i_up][j] = '.';
                        i_up -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn push_west(fields: &mut Vec<Vec<char>>) {
    for j in 1..fields[0].len() {
        for i in 0..fields.len() {
            let mut j_left = j;
            while j_left > 0 {
                if fields[i][j_left] == 'O' {
                    if fields[i][j_left - 1] == '.' {
                        fields[i][j_left - 1] = 'O';
                        fields[i][j_left] = '.';
                        j_left -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

fn push_south(fields: &mut Vec<Vec<char>>) {
    let mut i = fields.len() - 1;
    while i >= 0 {
        for j in 0..fields[i].len() {
            let mut i_down = i;
            while i_down < fields.len() - 1 {
                if fields[i_down][j] == 'O' {
                    if fields[i_down + 1][j] == '.' {
                        fields[i_down + 1][j] = 'O';
                        fields[i_down][j] = '.';
                        i_down += 1;
                    } else { break; }
                } else { break; }
            }
        }
        if i == 0{
            break;
        }
        i -= 1;
    }
}

fn push_east(fields: &mut Vec<Vec<char>>){
    let mut j = fields[0].len() - 1;
    while j >= 0{
        for i in 0..fields.len(){
            let mut j_right = j;
            while j_right < fields[0].len() - 1{
                if fields[i][j_right] == 'O' {
                    if fields[i][j_right + 1] == '.' {
                        fields[i][j_right + 1] = 'O';
                        fields[i][j_right] = '.';
                        j_right += 1;
                    } else { break; }
                } else { break; }
            }
        }
        if j == 0{
            break;
        }
        j -= 1;
    }
}

fn print_vec(f: &Vec<Vec<char>>){
    for i in 0..f.len(){
        for j in 0..f[i].len(){
            print!("{}",f[i][j]);
        }
        println!();
    }
    println!("************************************");
}
fn part2(lines: &Vec<String>) {
    let mut fields: Vec<Vec<char>> = Vec::new();
    let mut hash_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec: Vec<char> = Vec::new();
        for i in parts[0].chars() {
            tmp_vec.push(i);
        }
        fields.push(tmp_vec);
    }
    let mut id = 0;
    let mut t = 7;
    let mut beg = 0;
    let mut end = 0;
    while true {
        push_north(&mut fields);
        push_west(&mut fields);
        push_south(&mut fields);
        push_east(&mut fields);
        if hash_map.contains_key(&fields){
            beg = *hash_map.get(&fields).unwrap();
            end = id;
            break;
        }
        else {
            hash_map.insert(fields.clone(), id);
        }
        id += 1;
    }
    let mut cycle_length = end - beg;
    let modulo = 1000000000 % cycle_length;
    let mut value = 0;
    for i in beg..end{
        if i % cycle_length == modulo{
            value = i;
        }
    }
    let mut solve:Vec<Vec<char>> = Vec::new();
    for i in hash_map{
        if i.1 == value - 1{
            solve = i.0.clone();
            break;
        }
    }
    let n = solve.len();
    let mut sum = 0;
    for i in 0..solve.len() {
        for j in 0..solve[i].len() {
            if solve[i][j] == 'O' {
                sum += n - i;
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
    let day = "DAY14";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

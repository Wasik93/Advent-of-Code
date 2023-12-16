use std::char::MAX;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn traverse_part(table: &Vec<Vec<char>>, x: usize,y: usize, c:char) -> usize {
    let mut travel_array: Vec<Vec<(char)>> = Vec::new();
    let mut travel_hashmap: HashSet<(usize, usize, char)> = HashSet::new();
    for i in 0..table.len() {
        let mut tmp_vec = Vec::new();
        tmp_vec.resize(table[0].len(), '.');
        travel_array.push(tmp_vec);
    }

    let mut position = (x, y, c);
    let mut queue: VecDeque<(usize, usize, char)> = VecDeque::new();
    loop {
        let mut x = position.0;
        let mut y = position.1;
        let mut c = position.2;
        if table[x][y] == '@' || travel_hashmap.contains(&position) {
            if queue.is_empty() {
                break;
            } else {
                let mut v = queue.pop_front().unwrap();
                position.0 = v.0;
                position.1 = v.1;
                position.2 = v.2;
            }
            continue;
        }
        travel_array[x][y] = '#';
       // println!("{} {} {} {}", x, y, c, table[x][y]);
        travel_hashmap.insert((x,y,c));
        if c == '>' {
            if table[x][y] == '.' || table[x][y] == '-' {
                position.1 = y + 1;
            } else if table[x][y] == '|' {
                position.0 = x + 1;
                position.2 = 'v';
                queue.push_back((x - 1, y, '^'));
            } else if table[x][y] == '\\' {
                position.0 = x + 1;
                position.2 = 'v';
            } else if table[x][y] == '/' {
                position.0 = x - 1;
                position.2 = '^';
            }
        } else if c == '<' {
            if table[x][y] == '.' || table[x][y] == '-' {
                position.1 = y - 1;
            } else if table[x][y] == '|' {
                position.0 = x + 1;
                position.2 = 'v';
                queue.push_back((x - 1, y, '^'));
            } else if table[x][y] == '\\' {
                position.0 = x - 1;
                position.2 = '^';
            } else if table[x][y] == '/' {
                position.0 = x + 1;
                position.2 = 'v';
            }
        } else if c == '^' {
            if table[x][y] == '.' || table[x][y] == '|' {
                position.0 = x - 1;
            } else if table[x][y] == '-' {
                position.1 = y - 1;
                position.2 = '<';
                queue.push_back((x, y + 1, '>'));
            }
            else if table[x][y] == '\\'{
                position.1 = y - 1;
                position.2 = '<';
            }
            else if table[x][y] == '/'{
                position.1 = y + 1;
                position.2 = '>'
            }
        } else if c == 'v' {
            if table[x][y] == '.' || table[x][y] == '|' {
                position.0 = x + 1;
            } else if table[x][y] == '-' {
                position.1 = y - 1;
                position.2 = '<';
                queue.push_back((x, y + 1, '>'));
            }
            else if table[x][y] == '\\'{
                position.1 = y + 1;
                position.2 = '>'
            }
            else if table[x][y] == '/'{
                position.1 = y - 1;
                position.2 = '<';
            }
        }
    }
    let mut sum = 0;
    for i in travel_array {
        for j in i {
            if j == '#'{
                sum += 1;
            }
        }
    }
    return sum;
}

fn part1(lines: &Vec<String>) {
    let mut table: Vec<Vec<char>> = Vec::new();
    let mut tmp_vec = Vec::new();
    tmp_vec.resize(lines[0].len() + 2, '@');
    table.push(tmp_vec);

    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec = Vec::new();
        tmp_vec.push('@');
        for i in parts[0].chars() {
            tmp_vec.push(i);
        }
        tmp_vec.push('@');
        table.push(tmp_vec);
    }
    let mut tmp_vec = Vec::new();
    tmp_vec.resize(lines[0].len() + 2, '@');
    table.push(tmp_vec);

    println!("{}", traverse_part(&table, 1, 1, '>'));
}

fn max_4(d0:usize, d1:usize, d2:usize, d3:usize) -> usize{
    let x = max(d0, d1);
    let y = max(d2,d3);
    return max(x,y);
}

fn part2(lines: &Vec<String>) {
    let mut table: Vec<Vec<char>> = Vec::new();
    let mut tmp_vec = Vec::new();
    tmp_vec.resize(lines[0].len() + 2, '@');
    table.push(tmp_vec);

    for line in lines {
        let mut parts: Vec<_> = line.split_whitespace().collect();
        let mut tmp_vec = Vec::new();
        tmp_vec.push('@');
        for i in parts[0].chars() {
            tmp_vec.push(i);
        }
        tmp_vec.push('@');
        table.push(tmp_vec);
    }
    let mut tmp_vec = Vec::new();
    tmp_vec.resize(lines[0].len() + 2, '@');
    table.push(tmp_vec);

    let mut max_val = 0;
    let mut n = table[0].len() - 2;
    for i in 1..table.len() - 1{
        let d0 = traverse_part(&table, i, 1, '>');
        let d1 = traverse_part(&table, i, 1, 'v');
        let d2 = traverse_part(&table, i, 1, '^');
        let d3 = traverse_part(&table, i, 1, '<');
        let m = max_4(d0,d1,d2,d3);
        if max_val < m {
            max_val = m;
        }
        let d0 = traverse_part(&table, i, n, '>');
        let d1 = traverse_part(&table, i, n, 'v');
        let d2 = traverse_part(&table, i, n, '^');
        let d3 = traverse_part(&table, i, n, '<');
        let m = max_4(d0,d1,d2,d3);
        if max_val < m {
            max_val = m;
        }
    }
    n = table.len() - 2;
    for j in 1..table[0].len() - 1{
        let d0 = traverse_part(&table, 1, j, '>');
        let d1 = traverse_part(&table, 1, j, 'v');
        let d2 = traverse_part(&table, 1, j, '^');
        let d3 = traverse_part(&table, 1, j, '<');
        let m = max_4(d0,d1,d2,d3);
        if max_val < m {
            max_val = m;
        }
        let d0 = traverse_part(&table, n, j, '>');
        let d1 = traverse_part(&table, n, j, 'v');
        let d2 = traverse_part(&table, n, j, '^');
        let d3 = traverse_part(&table, n, j, '<');
        let m = max_4(d0,d1,d2,d3);
        if max_val < m {
            max_val = m;
        }
    }
    println!("{}",max_val);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let day = "DAY16";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

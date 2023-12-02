use std::f32::consts::E;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut sum: u32 = 0;
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for line in lines {
        first = 0;
        last = 0;
        let mut f = false;
        let mut vec: Vec<char> = line.chars().collect();
        for i in vec {
            if i.is_numeric() {
                if !f {
                    first = i as u32 - '0' as u32;
                    f = true;
                }
                last = i as u32 - '0' as u32;
            }
        }
        sum += first*10 + last;
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut numbers: Vec<&str> = Vec::new();
    numbers.push("one");
    numbers.push("two");
    numbers.push("three");
    numbers.push("four");
    numbers.push("five");
    numbers.push("six");
    numbers.push("seven");
    numbers.push("eight");
    numbers.push("nine");
    let mut sum = 0;
    for line in lines {
        let mut index_last = 0;
        let mut index_first = line.len();
        let mut value_first = 0;
        let mut value_last = 0;
        let mut id = 0;
        let mut f = false;
        let mut vec: Vec<char> = line.chars().collect();
        for i in vec {
            if i.is_numeric(){
                if !f {
                    index_first = id;
                    value_first = i as u32 - '0' as u32;
                    f = true;
                }
                index_last = id;
                value_last = i as u32 - '0' as u32;
            }
            id += 1;
        }
        id = 1;
        for number in &numbers {
            let indices: Vec<_> = line.match_indices(number).collect();
            for index in indices {
                if index.0 < index_first {
                    index_first = index.0;
                    value_first = id as u32;
                }
                if index.0 > index_last {
                    index_last = index.0;
                    value_last = id as u32;
                }
            }
            id += 1;
        }
        sum += value_first * 10 + value_last;
    }
    println!("{}",sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn main() {
    let day = "DAY1";
    let lines = lines_from_file("src/".to_owned() + day + ".txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn part1(lines: &Vec<String>) {
    let mut sum = 0;
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut id = 1;
    for line in lines {
        let mut split_line = line.clone();
        let parts: Vec<_> = split_line.split(|c| c == ':' || c == ';').collect();
        let mut f = true;
        for part in parts {
            if !f {
                break;
            }
            let games: Vec<_> = part.split(' ').collect();
            let mut idg = 0;
            let mut numero = 0;
            for game in games {
                if !f {
                    break;
                }
                f = true;
                if idg == 0 {
                    idg += 1;
                    continue;
                }
                if idg % 2 == 1 {
                    numero = game.parse().unwrap();
                } else {
                    if game.starts_with('r') {
                        if numero > max_red {
                            f = false;
                            break;
                        }
                    }
                    if game.starts_with('b') {
                        if numero > max_blue {
                            f = false;
                            break;
                        }
                    }
                    if game.starts_with('g') {
                        if numero > max_green {
                            f = false;
                            break;
                        }
                    }
                }
                idg += 1;
            }
        }
        if f {
            sum += id;
        }
        id += 1;
    }
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let mut split_line = line.clone();
        let parts: Vec<_> = split_line.split(|c| c == ':' || c == ';').collect();
        for part in parts {
            let games: Vec<_> = part.split(' ').collect();
            let mut idg = 0;
            let mut numero = 0;
            for game in games {
                if idg == 0 {
                    idg += 1;
                    continue;
                }
                if idg % 2 == 1 {
                    numero = game.parse().unwrap();
                } else {
                    if game.starts_with('r') {
                        if numero > min_red {
                            min_red = numero;
                        }
                    }
                    if game.starts_with('b') {
                        if numero > min_blue {
                            min_blue = numero;
                        }
                    }
                    if game.starts_with('g') {
                        if numero > min_green {
                            min_green = numero;
                        }
                    }
                }
                idg += 1;
            }
        }
        sum += min_green * min_blue * min_red;
        println!("G{}, B{}, R{}", min_green, min_blue, min_red);
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
    let day = "DAY2";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

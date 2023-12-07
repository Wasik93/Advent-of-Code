use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    score: usize,
    cards: String,
    bid: usize,
    part: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score != other.score {
            return self.score.cmp(&other.score);
        }
        for i in 0..5 {
            let char1 = self.cards.chars().nth(i).unwrap();
            let char2 = other.cards.chars().nth(i).unwrap();
            let mut val1 = 0;
            let mut val2 = 0;
            if char1 == 'A' {
                val1 = 14;
            } else if char1 == 'K' {
                val1 = 13;
            } else if char1 == 'Q' {
                val1 = 12;
            } else if char1 == 'J' && self.part == 1 {
                val1 = 11;
            } else if char1 == 'J' && self.part == 2 {
                val1 = 1;
            } else if char1 == 'T' {
                val1 = 10;
            } else {
                val1 = char1 as usize - '0' as usize;
            }
            if char2 == 'A' {
                val2 = 14;
            } else if char2 == 'K' {
                val2 = 13;
            } else if char2 == 'Q' {
                val2 = 12;
            } else if char2 == 'J' && self.part == 1 {
                val2 = 11;
            } else if char2 == 'J' && self.part == 2 {
                val2 = 1;
            } else if char2 == 'T' {
                val2 = 10;
            } else {
                val2 = char2 as usize - '0' as usize;
            }
            if val1 == val2 {
                continue;
            }
            return val1.cmp(&val2);
        }
        self.bid.cmp(&other.bid)
    }
}

fn part1(lines: &Vec<String>) {
    let mut data: Vec<Hand> = Vec::new();
    for line in lines {
        let split_line = line.clone();
        let parts: Vec<_> = split_line.split_whitespace().collect();
        let mut hand = Hand { cards: parts[0].to_string(), bid: parts[1].parse().unwrap(), score: 0, part: 1 };
        let mut map: HashMap<char, usize> = HashMap::new();
        for i in hand.cards.chars() {
            if map.contains_key(&i) {
                map.insert(i, *map.get(&i).unwrap() + 1);
            } else {
                map.insert(i, 1);
            }
        }
        let mut if_3 = false;
        let mut if_2 = false;
        for card in &map {
            if *card.1 == 2 {
                if_2 = true;
            }
            if *card.1 == 3 {
                if_3 = true;
            }
            hand.score = max(hand.score, *card.1);
        }
        if hand.score == 5 {
            hand.score = 6;
        } else if hand.score == 4 {
            hand.score = 5;
        } else {
            if if_2 && if_3 {
                hand.score = 4;
            } else if if_3 {
                hand.score = 3;
            } else if if_2 {
                for card in &map {
                    if *card.1 == 2 {
                        if if_2 {
                            if_2 = false;
                        } else {
                            if_2 = true;
                        }
                    }
                }
                if if_2 {
                    hand.score = 2;
                } else {
                    hand.score = 1
                }
            } else {
                hand.score = 0;
            }
        }
        hand.part = 1;
        data.push(hand);
    }
    data.sort();
    let mut id = 1;
    let mut sum = 0;
    for hand in data {
        sum += id * hand.bid;
        id += 1;
    }
    println!("{}", sum);
}


fn part2(lines: &Vec<String>) {
    let mut data: Vec<Hand> = Vec::new();
    for line in lines {
        let mut jokers = 0;
        let split_line = line.clone();
        let parts: Vec<_> = split_line.split_whitespace().collect();
        let mut hand = Hand { cards: parts[0].to_string(), bid: parts[1].parse().unwrap(), score: 0, part: 2 };
        let mut map: HashMap<char, usize> = HashMap::new();
        for i in hand.cards.chars() {
            if i == 'J' {
                jokers += 1;
                continue;
            }
            if map.contains_key(&i) {
                map.insert(i, *map.get(&i).unwrap() + 1);
            } else {
                map.insert(i, 1);
            }
        }
        let mut if_3 = false;
        let mut if_2 = false;
        for card in &map {
            if *card.1 == 2 {
                if_2 = true;
            }
            if *card.1 == 3 {
                if_3 = true;
            }
            hand.score = max(hand.score, *card.1);
        }
        if hand.score == 5 {
            hand.score = 6;
        } else if hand.score == 4 {
            hand.score = 5;
        } else {
            if if_2 && if_3 {
                hand.score = 4;
            } else if if_3 {
                hand.score = 3;
            } else if if_2 {
                for card in &map {
                    if *card.1 == 2 {
                        if if_2 {
                            if_2 = false;
                        } else {
                            if_2 = true;
                        }
                    }
                }
                if if_2 {
                    hand.score = 2;
                } else {
                    hand.score = 1
                }
            } else {
                hand.score = 0;
            }
        }
        if jokers == 5 {
            hand.score = 6;
        }
        if jokers == 4 {
            hand.score = 6;
        }
        if jokers == 3 {
            if hand.score > 0 {
                hand.score = 6;
            } else {
                hand.score = 5;
            }
        }
        if jokers == 2 {
            if hand.score == 3 {
                hand.score = 6;
            } else if hand.score == 1 {
                hand.score = 5;
            } else if hand.score == 0 {
                hand.score = 3;
            }
        }
        if jokers == 1 {
            if hand.score == 5 {
                hand.score = 6;
            } else if hand.score == 4 {
                hand.score = 5;
            } else if hand.score == 3 {
                hand.score = 5;
            } else if hand.score == 2 {
                hand.score = 4;
            } else if hand.score == 1 {
                hand.score = 3;
            } else if hand.score == 0 {
                hand.score = 1;
            }
        }
        hand.part = 2;
        data.push(hand);
    }
    data.sort();
    let mut id = 1;
    let mut sum = 0;
    for hand in data {
        sum += id * hand.bid;
        println!("{} {} {}", id, hand.score, hand.cards);
        id += 1;
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
    let day = "DAY7";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    // part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}

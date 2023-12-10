use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;



#[derive(Clone)]
struct Node {
    edge_up: bool,
    edge_down: bool,
    edge_left: bool,
    edge_right: bool,
    distance: usize,
    marker: char,
    visited: bool,
}

impl Node {
    fn new(u: bool, d: bool, l: bool, r: bool) -> Node {
        return Node { edge_up: u, edge_down: d, edge_left: l, edge_right: r, distance: 0, marker: ' ', visited: false };
    }
}


fn create_node(pipe: char) -> Node {
    return match pipe {
        'F' => { Node::new(false, true, false, true) }
        '|' => { Node::new(true, true, false, false) }
        'J' => { Node::new(true, false, true, false) }
        'L' => { Node::new(true, false, false, true) }
        '7' => { Node::new(false, true, true, false) }
        '-' => { Node::new(false, false, true, true) }
        'S' => { Node::new(true, true, false, false) }
        _ => { Node::new(false, false, false, false) }
    };
}

fn dfs(graph: &mut Vec<Vec<Node>>, i_in: usize, j_in: usize) {
    let mut i = i_in;
    let mut j = j_in;
    let mut f = true;

    while f || i != i_in || j != j_in {
        f = false;
        graph[i][j].visited = true;
        if graph[i][j].edge_up {
            if !graph[i - 1][j].visited {
                graph[i - 1][j].visited = true;
                graph[i - 1][j].distance = graph[i][j].distance + 1;
                graph[i - 1][j].marker = 'X';
                i = i - 1;
                continue;
            }
        }
        if graph[i][j].edge_down {
            if !graph[i + 1][j].visited {
                graph[i + 1][j].visited = true;
                graph[i + 1][j].distance = graph[i][j].distance + 1;
                graph[i + 1][j].marker = 'X';
                i = i + 1;
                continue;
            }
        }
        if graph[i][j].edge_right {
            if !graph[i][j + 1].visited {
                graph[i][j + 1].visited = true;
                graph[i][j + 1].distance = graph[i][j].distance + 1;
                graph[i][j + 1].marker = 'X';
                j = j + 1;
                continue;
            }
        }
        if graph[i][j].edge_left {
            if !graph[i][j - 1].visited {
                graph[i][j - 1].visited = true;
                graph[i][j - 1].distance = graph[i][j].distance + 1;
                graph[i][j - 1].marker = 'X';
                j = j - 1;
                continue;
            }
        }
        break;
    }
}

fn dfs2(graph: &mut Vec<Vec<Node>>, i_in: usize, j_in: usize) {
    let mut i = i_in;
    let mut j = j_in;
    let mut f = true;

    graph[i_in + 1][j_in].visited = true;
    graph[i_in - 1][j_in].visited = true;

    while f || i != i_in || j != j_in {
        f = false;
        graph[i][j].visited = true;
        if graph[i][j].edge_up {
            if !graph[i - 2][j].visited {
                graph[i - 2][j].visited = true;
                graph[i - 2][j].marker = 'X';
                graph[i - 1][j].marker = 'X';
                graph[i - 1][j].visited = true;
                i = i - 2;
                continue;
            }
        }
        if graph[i][j].edge_down {
            if !graph[i + 2][j].visited {
                graph[i + 2][j].visited = true;
                graph[i + 2][j].marker = 'X';
                graph[i + 1][j].marker = 'X';
                graph[i + 1][j].visited = true;
                i = i + 2;
                continue;
            }
        }
        if graph[i][j].edge_right {
            if !graph[i][j + 2].visited {
                graph[i][j + 2].visited = true;
                graph[i][j + 2].marker = 'X';
                graph[i][j + 1].marker = 'X';
                graph[i][j + 1].visited = true;
                j = j + 2;
                continue;
            }
        }
        if graph[i][j].edge_left {
            if !graph[i][j - 2].visited {
                graph[i][j - 2].visited = true;
                graph[i][j - 2].marker = 'X';
                graph[i][j - 1].marker = 'X';
                graph[i][j - 1].visited = true;
                j = j - 2;
                continue;
            }
        }
        break;
    }
}

fn bfs(graph: &mut Vec<Vec<Node>>) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let n = graph.len();
    for i in 0..n {
        if !graph[i][0].visited {
            graph[i][0].visited = true;
            queue.push_back((i, 0));
        }
    }
    for i in 0..n {
        if !graph[i][n - 1].visited {
            graph[i][n - 1].visited = true;
            queue.push_back((i, n - 1));
        }
    }
    for j in 0..n {
        if !graph[0][j].visited {
            graph[0][j].visited = true;
            queue.push_back((0, j));
        }
    }
    for j in 0..n {
        if !graph[n - 1][j].visited {
            graph[n - 1][j].visited = true;
            queue.push_back((n - 1, j));
        }
    }
    while !queue.is_empty() {
        let mut v = queue.pop_front().unwrap();
        let i = v.0;
        let j = v.1;
        graph[i][j].marker = 'O';
        if i != 0 {
            if !graph[i - 1][j].visited {
                graph[i - 1][j].visited = true;
                queue.push_back((i - 1, j));
            }
        }
        if i != n - 1 {
            if !graph[i + 1][j].visited {
                graph[i + 1][j].visited = true;
                queue.push_back((i + 1, j));
            }
        }
        if j != 0 {
            if !graph[i][j - 1].visited {
                graph[i][j - 1].visited = true;
                queue.push_back((i, j - 1));
            }
        }
        if j != n - 1 {
            if !graph[i][j + 1].visited {
                graph[i][j + 1].visited = true;
                queue.push_back((i, j + 1));
            }
        }
    }
}

fn part1(lines: &Vec<String>) {
    let mut graph: Vec<Vec<Node>> = Vec::new();
    let mut start_i = 0;
    let mut start_j = 0;
    let mut i = 0;
    for line in lines {
        let mut j = 0;
        let mut tmp_vec: Vec<Node> = Vec::new();
        let parts: Vec<_> = line.split_whitespace().collect();
        for pipe in parts[0].chars() {
            tmp_vec.push(create_node(pipe));
            if pipe == 'S' {
                start_i = i;
                start_j = j;
            }
            j += 1;
        }
        i += 1;
        graph.push(tmp_vec);
    }
    dfs(&mut graph, start_i, start_j);

    let mut max_val = 0;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if max_val < graph[i][j].distance {
                max_val = graph[i][j].distance;
            }
        }
    }
    println!("{}", (max_val + 1) / 2);
}

fn part2(lines: &Vec<String>) {
    let mut graph: Vec<Vec<Node>> = Vec::new();
    let mut start_i = 0;
    let mut start_j = 0;
    let mut i = 0;
    let mut n = lines[0].len();
    for line in lines {
        let mut j = 0;
        let mut tmp_vec: Vec<Node> = Vec::new();
        let parts: Vec<_> = line.split_whitespace().collect();
        for pipe in parts[0].chars() {
            tmp_vec.push(create_node(pipe));
            if pipe == 'S' {
                start_i = i;
                start_j = j;
            }
            j += 1;
            tmp_vec.push(create_node('.'));
            j += 1;
        }
        graph.push(tmp_vec);
        i += 1;
        let mut tmp_vec: Vec<Node> = Vec::new();
        for id in 0..2 * n {
            tmp_vec.push(create_node('.'));
        }
        graph.push(tmp_vec);
        i += 1;
    }

    dfs2(&mut graph, start_i, start_j);

    let mut sum_val = 0;
    bfs(&mut graph);
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if i % 2 == 0 && j % 2 == 0 &&
                !graph[i][j].visited
            {
                sum_val += 1;
            }
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
    let day = "DAY10";
    let lines = lines_from_file("src/test.txt");

    println!("{} - {}", day, "PART 1");
    part1(&lines);
    println!("{} - {}", day, "PART 2");
    part2(&lines);
}
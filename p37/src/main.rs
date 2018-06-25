extern crate itertools;

use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let (height, width) = read_header();
    let pattern = read_pattern(height, width);

    let mut search_queue = VecDeque::<Position>::new();
    let start_pos = find_start(&pattern).expect("no start found");
    search_queue.push_front(start_pos);
    let mut visited = HashSet::<i64>::new();

    while !search_queue.is_empty() {
        let current_pos = search_queue.pop_back().unwrap();
        visited.insert(current_pos.to_key(width)); // mark current_pos as visited

        // search up, down, left, and right
        for (x, y) in vec![(current_pos.x, current_pos.y-1), (current_pos.x, current_pos.y+1), (current_pos.x-1, current_pos.y), (current_pos.x+1, current_pos.y)] {
            if !(0 <= x && x < width as i64 && 0 <= y && y < height as i64) { continue; }
            match pattern[y as usize][x as usize] {
                'S' => (),
                '#' => (),
                '.' => {
                    if !visited.contains(&position2key(x, y, width)) {
                        search_queue.push_front(Position{x: x, y: y, least_distance: current_pos.least_distance+1});
                    }
                },
                'G' => {
                    println!("shortest path length is {}", current_pos.least_distance + 1);
                    return;
                },
                _ => panic!("unsupported character {}", pattern[y as usize][x as usize]),
            }
        }
    }

    println!("no route to the goal is found");
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    least_distance: i64,
}

impl Position {
    /// map (x, y) to y * width + x
    fn to_key(&self, width: usize) -> i64 {
        self.x + self.y * width as i64
    }

}

fn position2key(x: i64, y: i64, width: usize) -> i64 {
    x + y * width as i64
}

fn find_start(pattern: &Vec<Vec<char>>) -> Option<Position> {
    for (y, line) in pattern.iter().enumerate() {
        for (x, p) in line.iter().enumerate() {
            if *p == 'S' {
                return Some(Position{x: x as i64, y: y as i64, least_distance: 0});
            }
        }
    }
    None
}

/// "10 12" -> (10, 12)
fn read_header() -> (usize, usize) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read header");
    s.trim().split_whitespace().map(|n| n.parse::<usize>().expect("failed parsing header")).next_tuple().unwrap()
}

fn read_pattern(height: usize, width: usize) -> Vec<Vec<char>> {
    let mut pattern = Vec::with_capacity(height);
    for _ in 0..height {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("failed to read a line in a pattern");
        let line = s.trim().chars().collect::<Vec<char>>();
        assert_eq!(line.len(), width, "invalid line in a patter, got {} chars", line.len());
        pattern.push(line);
    }
    pattern
}
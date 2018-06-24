extern crate itertools;

use itertools::Itertools;

fn main() {
    let (height, width) = read_header();
    let garden = read_pattern(height, width);

    // convert to list of Positions
    let mut lakes = Vec::<Position>::with_capacity(height * width);
    for (i, line) in garden.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'w' || *c == 'W' {
                lakes.push(Position{x: j as i64, y: i as i64});
            }
        }
    }

    let mut count: usize = 0; // increment every time dfs_search_list is emptied
    let mut dfs_search_list = Vec::<Position>::new();
    while !lakes.is_empty() {
        dfs_search_list.push(lakes.pop().unwrap());
        while !dfs_search_list.is_empty() {
            let start = dfs_search_list.pop().unwrap();
            let next_positions = search_next_position(start, &lakes);
            for p in next_positions {
                match find_first_index(&p, &lakes) {
                    Some(idx) => dfs_search_list.push(lakes.remove(idx)),
                    None => panic!("{:?} not found in lakes", p),
                }
            }
        }
        count = count + 1;
    }

    println!("{} lakes", count)
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn is_next(&self, p: &Self) -> bool {
        (self.x - p.x).abs() <= 1 && (self.y - p.y).abs() <= 1
    }
}

fn find_first_index(val: &Position, vs: &Vec<Position>) -> Option<usize> {
    for (i, v) in vs.iter().enumerate() {
        if val == v {
            return Some(i);
        }
    }
    None
}

fn search_next_position(base: Position, list: &Vec<Position>) -> Vec<Position> {
    list.iter().cloned().filter(|p| base != *p && base.is_next(p)).collect()
}

// read "10 12" and return (10, 12)
fn read_header() -> (usize, usize) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|v| v.parse().ok().unwrap()).next_tuple().unwrap()
}

fn read_pattern(height: usize, width: usize) -> Vec<Vec<char>> {
    let mut garden = Vec::with_capacity(height);
    for _ in 0..height {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let line = s.trim().chars().collect::<Vec<char>>();
        if line.len() != width {
            panic!("wrong size of garden recieved")
        }
        garden.push(line)
    }
    garden
}
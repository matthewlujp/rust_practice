extern crate itertools;

use itertools::Itertools;

fn main() {
    let (height, width) = read_header();
    let mut garden = read_pattern(height, width);

    let mut count = 0 as usize;
    while let Some((x, y)) = find_first_water(&garden){
        dfs(x, y, &mut garden);
        count = count + 1;
    }

    println!("{} lakes found", count);
}

fn dfs(x: usize, y: usize, lake: &mut Vec<Vec<char>>) {
    let mut search_stack = Vec::new();
    search_stack.push((x, y));
    *(lake.get_mut(y).unwrap().get_mut(x).unwrap()) = '.';

    while let Some((x, y)) = search_stack.pop() {
        // search adjacent w
        for i in vec![y as i64 -1, y as i64, y as i64 +1] {
            if i < 0 { continue; }
            match lake.get_mut(i as usize) {
                Some(v) => {
                    for j in vec![x as i64 -1, x as i64, x as i64 +1] {
                        if j < 0 { continue; }
                        match v.get_mut(j as usize) {
                            Some(c) => {
                                if *c == 'w' || *c == 'W' {
                                    search_stack.push((j as usize, i as usize));
                                    *c = '.';
                                }
                            },
                            None => continue,
                        }
                    }
                },
                None => continue,
            }
        }
    }
}

fn find_first_water(garden: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, v) in garden.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == 'w' || *c == 'W' {
                return Some((j, i));
            }
        }
    }
    None
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
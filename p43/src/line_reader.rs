use std;

/// Read a value in a line
pub fn read_val<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}


/// Read values in a line separated by spaces and return as a vector
pub fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|v| v.parse().ok().unwrap()).collect()
}
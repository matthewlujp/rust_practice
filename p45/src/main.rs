mod line_reader;
use line_reader::{read_val, read_vec};
use std::iter::FromIterator;

fn main() {
    println!("Hello, world!");
}

fn rearrange(org_str: &str) -> String {
    let mut org_v = org_str.chars().collect::<Vec<char>>();
    let mut new_v = Vec::<char>::with_capacity(org_v.len());

    while org_v.len() > 1 {
        if org_v[0] < org_v[org_v.len()-1] {
            new_v.push(org_v.remove(0));
        } else if org_v[0] > org_v[org_v.len()-1] {
            new_v.push(org_v.pop().unwrap());
        } else { // head_c == tail_c
            let i = 1;
            let mut use_head = true;
            while i as f64 <= org_v.len() as f64 / 2.0 {
                if org_v[i] < org_v[org_v.len() - i] {
                    use_head = true;
                    break;
                } else if org_v[i] > org_v[org_v.len() - i] {
                    use_head = false;
                    break;
                }
            }

            new_v.push(if use_head {org_v.remove(0)} else {org_v.pop().unwrap()})
        }
    }

    new_v.push(org_v.pop().unwrap()); // the last one
    return String::from_iter(new_v);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rearrange() {
        assert_eq!(rearrange("ACDBCB"), String::from("ABCBCD"));
        assert_eq!(rearrange("CBABBC"), String::from("CBABBC"));
    }
}
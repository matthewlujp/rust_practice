fn main() {
    let n = read_val::<usize>();
    let a = read_vec::<i64>();
    let k = read_val::<i64>();

    let max_k = a.iter().cloned().fold(0, |sum, v| sum + v);
    let minus_sum = a.iter().cloned().fold(0, |sum, v| sum + std::cmp::min(0, v));
    let _k: usize = (max_k + minus_sum) as usize; // shift minus_sum
    let _a = a.iter().map(|v| *v + minus_sum).collect::<Vec<i64>>();

    // dp[i][k] = make k using a_0, ... ,a_i
    let mut dp = Vec::with_capacity(n);
    for _ in 0..n {
        dp.push(vec![false; _k+1]);
        *(dp.last_mut().unwrap().get_mut(0).unwrap()) = true;
    }
    
    for i in 1..n {
        for j in 0.._k+1 {
            if j as i64 - _a[i] >= 0 {
                *(dp.get_mut(i).unwrap().get_mut(j).unwrap()) = dp[i-1][j] || dp[i-1][j - _a[i] as usize];
            }
        }
    }

    if !dp.last().unwrap()[(k + minus_sum) as usize] {
        println!("no");
        return;
    }

    let mut combination = Vec::with_capacity(n);
    let mut j = (k + minus_sum) as usize;
    for i in (1..n).rev() {
        if dp[i-1][j] {
            // do not use a[i] to make j
            if i-1 == 0 && j > 0 {
                combination.push(a[0]);
            }
        } else if j as i64 - _a[i] >= 0  && dp[i-1][j-_a[i] as usize] {
            // use a[i] to make j
            combination.push(a[i]);
            j = j - _a[i] as usize;
            if j == 0 {
                break;
            }
        } else {
            println!("no failed back trace dp result");
            return;
        }
    }
    println!("yes ({:?})", combination);
}

fn read_val<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|v| v.parse().ok().unwrap()).collect()
}
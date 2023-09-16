use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        m: usize,
        s1: String,
        s2: String,
        s3: String,
    }
    let mut arr = vec![0; 10];
    let mut s1_map: HashSet<usize> = HashSet::new();
    let mut s2_map: HashSet<usize> = HashSet::new();
    let mut s3_map: HashSet<usize> = HashSet::new();
    for i in 0..m {
        if let Ok(s) = s1.chars().nth(i).unwrap().to_string().parse::<usize>() {
            if s1_map.insert(s) {
                arr[s] += 1;
                if arr[s] == 3 {
                    println!("{}", i);
                    return;
                }
            }
        }

        if let Ok(s) = s2.chars().nth(i).unwrap().to_string().parse::<usize>() {
            if s2_map.insert(s) {
                arr[s] += 1;
            }
            if arr[s] == 3 {
                println!("{}", i);
                return;
            }
        }

        if let Ok(s) = s3.chars().nth(i).unwrap().to_string().parse::<usize>() {
            if s3_map.insert(s) {
                arr[s] += 1;
            }
            if arr[s] == 3 {
                println!("{}", i);
                return;
            }
        }
    }
    println!("-1");
}

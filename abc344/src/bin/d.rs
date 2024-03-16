use proconio::{input, input_interactive, marker::Chars};
use std::cmp::min;
use std::collections::HashMap;

const INF: usize = 1_000_000_000;

fn kmp(a: &str, b: &str) -> Vec<usize> {
    let n = a.len();
    let m = b.len();
    let mut pi = vec![0; m];
    for i in 1..m {
        let mut j = pi[i - 1];
        while j > 0 && a.chars().nth(i) != b.chars().nth(j) {
            j = pi[j - 1];
        }
        if a.chars().nth(i) == b.chars().nth(j) {
            pi[i] = j + 1;
        }
    }
    pi
}

fn main() {
    input! {
        t: String,
        n: usize,
    }
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(t, 0);

    for _ in 0..n {
        input_interactive! {
            a: usize,
            s: [String; a]
        }
        println!("{:?}", s);

        for ss in s.iter() {
            let newKey = map
        }
    }

    // let lcp = kmp(a, b).last().unwrap();
    // let ans = &a[lcp..];
}

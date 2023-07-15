use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut hs = HashSet::new();
    for item in s.iter().cloned() {
        let normalized = normalize_string(item);
        hs.insert(normalized.clone());
    }
    print!("{}", hs.len())
}

fn normalize_string(input: String) -> String {
    let reversed = input.chars().rev().collect::<String>();
    if input < reversed {
        input + &reversed
    } else {
        reversed + &input
    }
}

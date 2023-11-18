use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut hash: HashMap<char, usize> = HashMap::new();
    let mut count = 1;

    for i in 0..n {
        if i < n - 1 && s[i] == s[i + 1] {
            count += 1;
        } else {
            if let Some(val) = hash.get(&s[i]) {
                hash.insert(s[i], *val.max(&count));
            } else {
                hash.insert(s[i], count);
            }
            count = 1;
        }
    }
    // hashのvalueの総和を取得
    let mut sum = 0;
    for (_, v) in hash {
        sum += v;
    }
    println!("{:?}", sum);
}

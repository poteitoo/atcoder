use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    for ss in s.iter() {
        if let Some(val) = map.get_mut(ss) {
            *val += 1;
        } else {
            map.insert(*ss, 1);
        }
    }

    let mut ans = vec![0; 108];
    for val in map.values() {
        ans[*val] += 1;
    }
    for aa in ans.iter() {
        if *aa != 0 && *aa != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

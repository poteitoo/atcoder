use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut arr = a
        .iter()
        .enumerate()
        .map(|(i, a)| (i as isize + 1, a))
        .collect_vec();

    let mut map = BTreeMap::new();
    for (key, value) in arr.iter().cloned() {
        map.insert(*value, key);
    }
    arr.sort_by_key(|k| k.1);

    let mut idx = arr[0].0;
    print!("{}", idx);
    for _ in 1..n {
        let res = map.get(&idx).unwrap().clone();
        print!(" {}", res);
        idx = res;
    }
}

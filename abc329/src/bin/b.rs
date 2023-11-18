use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let hash: HashSet<usize> = HashSet::from_iter(a);
    let mut hash = hash.into_iter().collect_vec();
    hash.sort_unstable();

    println!("{:?}", hash[hash.len() - 2]);
}

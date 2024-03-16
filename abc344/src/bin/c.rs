use std::collections::HashSet;

use amplify::confinement::Collection;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }

    let mut memo = HashSet::new();
    for aa in a.iter() {
        for bb in b.iter() {
            for cc in c.iter() {
                memo.push(*aa + *bb + *cc);
            }
        }
    }

    for xx in x.iter() {
        if memo.contains(xx) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    }

    let mut p_vec: Vec<usize> = Vec::new();
    let mut f_vec: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            f: [usize; c]
        }
        p_vec.push(p);
        f_vec.push(f.clone());
    }

    for i in 0..n - 1 {
        let pi = p_vec[i];
        let fi: HashSet<_> = f_vec[i].clone().into_iter().collect();
        for j in 0..n {
            if i == j {
                continue;
            }

            let pj = p_vec[j];
            let fj: HashSet<_> = f_vec[j].clone().into_iter().collect();
            let diff = fj.difference(&fi);
            if pi < pj {
                continue;
            }
            if !fi.is_subset(&fj) {
                continue;
            }
            if pi > pj || diff.count() > 0 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

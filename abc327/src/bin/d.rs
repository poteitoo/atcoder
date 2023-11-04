use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        a: [Usize1; m],
        b: [Usize1; m],
    }
    let mut uf = UnionFind::new(n * 2);
    for i in 0..m {
        uf.union(a[i], b[i] + n);
        uf.union(a[i] + n, b[i]);
    }

    for i in 0..n {
        if uf.equiv(i, i + n) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

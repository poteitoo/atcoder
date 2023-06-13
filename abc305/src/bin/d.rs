use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        q: usize,
        lr: [(usize, usize);q],
    }

    //fA[i] := A[i]までに何分寝たか
    let mut fa: Vec<usize> = vec![0; n + 1];
    for i in 1..n {
        if i % 2 == 0 {
            fa[i] = fa[i - 1] + (a[i] - a[i - 1]);
        } else {
            fa[i] = fa[i - 1];
        }
    }

    //f(x) := x分までに何分寝たか
    let f = |x: usize| -> usize {
        let j = a.binary_search(&x).unwrap_or_else(|j| j);
        let j = if j > 0 { j - 1 } else { j };
        fa[j] + (fa[j + 1] - fa[j]) / (a[j + 1] - a[j]) * (x - a[j])
    };

    for (l, r) in lr {
        println!("{}", f(r) - f(l));
    }
}

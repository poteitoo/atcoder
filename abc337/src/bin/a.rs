use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(usize, usize); n],
    }

    let mut t = 0;
    let mut a = 0;
    for (ti, ai) in ta {
        t += ti;
        a += ai;
    }
    let ans = if t > a {
        "Takahashi"
    } else if t == a {
        "Draw"
    } else {
        "Aoki"
    };
    println!("{}", ans)
}

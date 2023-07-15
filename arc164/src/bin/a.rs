use proconio::input;

fn base3(mut n: i64) -> i64 {
    let mut x = 0;
    while 0 < n {
        x += n % 3;
        n /= 3;
    }
    x
}

fn main() {
    input!(n: usize, nk: [(i64, i64); n]);
    for (n, k) in nk {
        let x = base3(n);
        if (n - k) % 2 == 0 && x <= k {
            println!("{}, {}", n, x);
            println!("Yes")
        } else {
            println!("{}, {}", n, x);
            println!("No")
        }
    }
}

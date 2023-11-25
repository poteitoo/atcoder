use proconio::input;

fn main() {
    input! {
        d: i64,
    }
    let mut y = 2_000_000;
    let mut r = i64::MAX;

    for x in 0..=2_000_000 {
        while y > 0 && x * x + y * y > d {
            y -= 1
        }
        r = r
            .min((x * x + y * y - d).abs())
            .min((x * x + (y + 1) * (y + 1) - d).abs());
    }
    println!("{}", r);
}

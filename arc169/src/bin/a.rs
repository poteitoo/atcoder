use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        p: [usize; n-1]
    }

    let mut distance = vec![0; n];
    for i in 2..=n {
        distance[i - 1] = distance[p[i - 2] - 1] + 1;
    }
    let mut sums = vec![0; n];
    for i in 0..n {
        sums[distance[i]] += a[i];
    }
    for sum in sums.iter().rev() {
        if *sum != 0 {
            println!("{}", if *sum > 0 { "+" } else { "-" });
            return;
        }
    }
    println!("0");
}

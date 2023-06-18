// 素数判定

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {n:isize}
    solve(n);
}

fn solve(n: isize) {
    let end = n.sqrt() / 6 + 1;
    println!("worst computation: {}", end * 2 + 2);

    if n % 2 == 0 {
        println!("n%2: {}", 2);
        return;
    }
    if n % 3 == 0 {
        println!("n%3: {}", 3);
        return;
    }

    for (i, j) in (1..=end).enumerate() {
        let d0 = 6 * j - 1;
        let d1 = 6 * j + 1;

        if n % d0 == 0 {
            println!("cou: {} d0: {}", i, d0);
            return;
        } else if n % d1 == 0 {
            println!("cou: {} d1: {}", i, d1);
            return;
        }
    }

    println!("prime number");
}

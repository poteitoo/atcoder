use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut primes = vec![true; n + 1];
    let mut ans = Vec::new();

    for num in 2..=n {
        if primes[num] {
            ans.push(num);

            let mut multiple = num * 2;
            while multiple <= n {
                primes[multiple] = false;
                multiple += num;
            }
        }
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}

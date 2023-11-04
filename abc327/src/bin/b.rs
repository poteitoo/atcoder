use num_bigint::BigInt;
use num_traits::Pow;
use proconio::input;

fn main() {
    input! {
        b: BigInt
    }

    for n in 1u32..=16 {
        let x: BigInt = n.into();
        let num = x.pow(n);
        if num.eq(&b) {
            println!("{}", n);
            return;
        }
    }
    println!("-1");
}

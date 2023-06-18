use proconio::input;

fn main() {
    input! {
        a: [usize;64],
    }

    let mut arr: [usize; 64] = [1; 64];

    for i in 1..64 {
        arr[i] = arr[i - 1] * 2;
    }

    let mut res = 0;
    for i in 0..64 {
        res += a[i] * arr[i];
    }

    println!("{}", res);
}

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        ss: [Chars;n]
    }

    let mut t = vec![0; d];
    for s in ss.iter().cloned() {
        let s = s
            .iter()
            .map(|v| if *v == 'o' { 1 } else { 0 })
            .collect_vec();

        t = t.iter().zip(s.iter()).map(|(&x, &y)| x + y).collect();
    }

    let mut max_length = 0;
    let mut current_length = 0;

    for num in t.iter().cloned() {
        if num == n {
            current_length += 1;
        } else {
            max_length = max_length.max(current_length);
            current_length = 0;
        }
    }
    println!("{}", max_length.max(current_length))
}

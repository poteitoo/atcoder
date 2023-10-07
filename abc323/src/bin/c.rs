use proconio::{input, marker::Chars};

fn min_elements_to_subtract(mut a: isize, arr: &Vec<isize>) -> Option<usize> {
    let mut count = 0;
    for &val in arr.iter() {
        a -= val;
        count += 1;
        if a < 0 {
            return Some(count);
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; m],
        s: [Chars; n]
    }
    let mut sums = vec![0; n];

    for (i, ss) in s.iter().enumerate() {
        for (j, sss) in ss.iter().enumerate() {
            if *sss == 'o' {
                sums[i] += a[j];
            }
        }
        sums[i] += i as isize + 1;
    }
    let max = sums.iter().max().unwrap();

    let mut aa = a;
    aa.sort_unstable_by(|a, b| b.cmp(a));

    for sum in &sums {
        if sum == max {
            println!("0");
            continue;
        }
        let ans = min_elements_to_subtract(max - sum, &aa).unwrap();
        println!("{}", ans);
    }
}

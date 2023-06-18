use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n:usize, p:[usize;n]}
    let res = solve(p);
    println!("{}", res);
}

fn solve(p: Vec<usize>) -> usize {
    println!("{:?}", p);

    let arr: Vec<isize> = p
        .iter()
        .enumerate()
        .map(|(i, n)| i as isize - *n as isize)
        .collect_vec();
    // arr.sort();
    // let max = arr.iter().max().unwrap();
    // let max = arr.last().unwrap();
    println!("{:?}", arr);
    // println!("max: {}", max);

    return arr.iter().filter(|&num| *num >= -1).count();
}

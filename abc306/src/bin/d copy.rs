use proconio::input;

fn dfs(i: usize, n: usize, sum: &mut usize, xy: &Vec<(usize, isize)>) -> usize {
    if i >= n {
        return *sum;
    }
    if i - 1 > 0 && xy[i].0 == 1 && xy[i - 1].0 == 1 {
        return 0;
    }

    if xy[i].0 == 1 {
        *sum = dfs(i, n, sum, xy) + *sum;
    } else {
        *sum = dfs(i + 1, n, sum, xy) + *sum;
    }

    return *sum;
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, isize); n]
    }

    let mut sum = 0;
    let res = dfs(0, n, &mut sum, &xy);
    println!("{}", res);
}

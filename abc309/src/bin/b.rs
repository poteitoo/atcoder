use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = a.clone();

    // 上
    for i in 1..n {
        ans[0][i] = a[0][i - 1];
    }
    // 下
    for i in 0..n - 1 {
        ans[n - 1][i] = a[n - 1][i + 1];
    }

    // 左
    for i in 0..n - 1 {
        ans[i][0] = a[i + 1][0];
    }
    // 右
    for i in 0..n - 1 {
        ans[i + 1][n - 1] = a[i][n - 1];
    }

    let cat = ans
        .iter()
        .map(|row| row.iter().map(|cell| cell.to_string()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", cat);
}

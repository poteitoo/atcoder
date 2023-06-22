use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize;m];n]
    }
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for row in a.iter() {
        let row_sum: usize = row.iter().sum();
        rows.push(row_sum);
    }

    for col in 0..m {
        let col_sum: usize = a.iter().map(|row| row[col]).sum();
        cols.push(col_sum);
    }

    for (i, row) in rows.iter().enumerate() {
        let mut res = Vec::new();
        for (j, col) in cols.iter().enumerate() {
            res.push(format!("{}", row + col - a[i][j]));
        }
        println!("{}", res.join(" "));
    }
}

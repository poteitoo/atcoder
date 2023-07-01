use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize;w];h],
        q: usize,
        abcd: [(usize,usize,usize,usize);q]
    }

    let mut xx = vec![vec![0_usize; w + 1]; h + 1];
    for iy in 0..h {
        for ix in 0..w {
            xx[iy + 1][ix + 1] = xx[iy + 1][ix] + x[iy][ix];
        }
        for ix in 0..w {
            xx[iy + 1][ix + 1] += xx[iy][ix + 1];
        }
    }
    for (a, b, c, d) in abcd.iter().cloned() {
        println!(
            "{}",
            xx[c][d] - xx[c][b - 1] - xx[a - 1][d] + xx[a - 1][b - 1]
        );
    }
}

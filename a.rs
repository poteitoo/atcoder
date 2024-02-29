use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    // input! {
    //     n: usize,
    //     m: usize,
    //     e: f32,
    //     mut pols: [[usize; n]; m],
    // }
    let n = 15;
    // let m = 2;
    // let e = 0.01;
    let pols: Vec<Vec<usize>> = vec![
        vec![
            19, 0, 3, 1, 1, 1, 3, 1, 6, 2, 0, 2, 1, 2, 2, 2, 3, 2, 4, 2, 5, 2, 6, 3, 1, 3, 2, 3, 3,
            3, 4, 3, 5, 4, 1, 4, 2, 4, 4,
        ],
        vec![
            19, 0, 1, 1, 1, 2, 1, 3, 1, 3, 2, 3, 3, 3, 4, 4, 1, 4, 2, 5, 0, 5, 1, 5, 2, 5, 3, 6, 1,
            6, 2, 6, 3, 7, 1, 7, 3, 8, 3,
        ],
    ];

    // ポリノミオを埋め込むことができるマスを探す
    let mut available_grid = HashSet::new();
    for pol in pols.iter() {
        // ２組のチャンクに分ける
        let pol = pol[1..].chunks(2).into_iter().collect_vec();
        // xy軸それぞれの最大値を取得
        let x_max = pol.iter().max_by_key(|a| a[1]).unwrap()[1];
        let y_max = pol.iter().max_by_key(|a| a[0]).unwrap()[0];

        for p in pol.iter() {
            for x in 0..n - x_max {
                for y in 0..n - y_max {
                    available_grid.insert((p[0] + y, p[1] + x));
                }
            }
        }
    }
    let mut grid = available_grid.iter().cloned().collect_vec();
    grid.sort_unstable();

    println!(
        "q {} {}",
        grid.len(),
        grid.iter().map(|(x, y)| format!("{} {}", x, y)).join(" ")
    );

    for (x, y) in grid.iter().cloned() {
        println!("q 1 {} {}", x, y);
        input! {
            v: usize,
        }
        if v == 0 {
            available_grid.remove(&(x, y));
        }
    }

    // 10チャンクごとに出力
    // let mut grid = available_grid.into_iter().collect_vec();
    // grid.sort();
    // for g in grid.chunks(10) {
    //     println!(
    //         "q {} {}",
    //         g.len(),
    //         g.iter().map(|(x, y)| format!("{} {}", x, y)).join(" ")
    //     )
    // }

    // for (x, y) in available_grid.iter() {}

    // let x_max = pol.iter().max_by_key(|a| a[1]);
    // let y_max = pol.iter().max_by_key(|a| a[0]);

    // println!("x: {:?}", x_max);
    // println!("y: {:?}", y_max);
    // println!("{:?}", pol);

    // println!("{} {}", pol[1..].len() / 2, pol[1..].iter().join(" "))
}

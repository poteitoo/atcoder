use itertools::iproduct;
use itertools::Itertools;
use proconio::input_interactive;
fn main() {
    input_interactive! {n:usize, m:usize, _ro:f64};
    input_interactive! {_oil:[[(usize,usize)];m]};
    let mut ans = vec![];
    for (i, j) in iproduct!(0..n, 0..n) {
        println!("q 1 {i} {j}");
        input_interactive! {v:usize};
        if v != 0 {
            ans.push([i, j]);
        }
    }
    println!("a {} {}", ans.len(), ans.into_iter().flatten().join(" "));
}

// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         _e: f32,
//         pols: Usize1,
//     }
//     println!("pols {:?}", pols);

//     // for p in pols {
//     //     println!("{:?}", p);
//     // }
//     // let num_of_yuden_cells = pols.iter().map(|a| a[0]).sum::<usize>();
//     // println!("num_of_yuden_cells: {}", num_of_yuden_cells);

//     // // ポリノミオを埋め込むことができるマスを探す
//     // let mut available_grid = HashSet::new();
//     // for pol in pols.iter() {
//     //     // ２組のチャンクに分ける
//     //     let pol = pol[1..].chunks(2).into_iter().collect_vec();
//     //     // xy軸それぞれの最大値を取得
//     //     let x_max = pol.iter().max_by_key(|a| a[1]).unwrap()[1];
//     //     let y_max = pol.iter().max_by_key(|a| a[0]).unwrap()[0];

//     //     for p in pol.iter() {
//     //         for x in 0..n - x_max {
//     //             for y in 0..n - y_max {
//     //                 available_grid.insert((p[0] + y, p[1] + x));
//     //             }
//     //         }
//     //     }
//     // }

//     // // 全てのマスを試す
//     // let mut cou = 0;
//     // let mut grid = available_grid.iter().cloned().collect_vec();
//     // grid.sort_unstable();
//     // println!("{:?}", grid);

//     // for (x, y) in grid.iter().cloned() {
//     //     println!("q 1 {} {}", x, y);
//     //     input! {
//     //         v: usize,
//     //     }
//     //     if v == 0 {
//     //         available_grid.remove(&(x, y));
//     //     } else {
//     //         cou += v;
//     //         println!("cou: {}, val: {}", cou, v)
//     //     }

//     //     if cou == num_of_yuden_cells {
//     //         break;
//     //     }
//     // }

//     // println!(
//     //     "a {} {}",
//     //     available_grid.len(),
//     //     available_grid
//     //         .iter()
//     //         .map(|(x, y)| format!("{} {}", x, y))
//     //         .join(" ")
//     // );
// }

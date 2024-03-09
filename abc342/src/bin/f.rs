use std::collections::{BinaryHeap, HashMap};

use proconio::{input, input_interactive};

fn main() {
    input! {
      n: usize,
      m: usize,
    }
    let mut edges: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();
    for _ in 0..m {
        input! {
          li: usize,
          di: usize,
          ki: usize,
          ci: usize,
          ai: usize,
          bi: usize,
        }
        println!("? {} {}", ai + 1, bi + 1);
        for t in 0..ki {
            let u = li + t * di;
            let v = ai + 1;
            let w = ci;
            edges.entry(u).or_default().push((v, w, bi));
        }
    }

    // 駅間の距離を計算
    let mut distance: Vec<usize> = vec![usize::max_value(); n];
    distance[0] = 0;
    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    pq.push((0, 0));
    while let Some((cost, u)) = pq.pop() {
        if cost > distance[u] {
            continue;
        }
        for (v, w, _) in &edges[&u] {
            let new_cost = cost + w;
            if new_cost < distance[*v] {
                distance[*v] = new_cost;
                pq.push((new_cost, *v));
            }
        }
    }

    // 最終到達時刻を計算
    let mut latest_time: Vec<usize> = vec![0; n];
    for i in 0..n {
        for (v, w, _) in &edges[&i] {
            latest_time[*v] = std::cmp::max(latest_time[*v], latest_time[i] + w);
        }
    }

    // 出力
    for i in 1..n {
        println!(
            "{}",
            if latest_time[i] == usize::max_value() {
                0
            } else {
                latest_time[i]
            }
        );
    }
}

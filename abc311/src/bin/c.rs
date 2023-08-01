use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let cand: HashSet<_> = a.iter().cloned().collect();
    let mut graph: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        if cand.contains(&(i + 1)) {
            graph.insert(i + 1, a[i]);
        }
    }

    for (key, val) in graph.iter() {
        let origin = *key;
        solve(&graph, &origin, val)
    }

    fn solve(graph: &HashMap<usize, usize>, origin: &usize, next: &usize) {
        if origin == next {
            println!("{} {}", origin, next);
            return;
        }
        if let Some(next_val) = graph.get(next) {
            println!("{} {}", origin, next_val);
            solve(graph, origin, next_val);
        }
    }
}

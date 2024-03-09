use std::collections::HashSet;

fn main() {
    // 入力
    let n: usize = read_line();
    let t: usize = read_line();
    let mut a: Vec<usize> = Vec::with_capacity(n);
    let mut b: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(read_line());
        b.push(read_line());
    }

    // 各時点における得点の種類数を格納する配列
    let mut counts: Vec<usize> = Vec::with_capacity(t + 1);

    // 初期状態の得点の種類数を計算
    counts[0] = count_unique_values(&a);

    // 各イベント処理
    for i in 0..t {
        // イベント後の各選手の得点
        let mut next_scores: Vec<usize> = Vec::with_capacity(n);
        for j in 0..n {
            next_scores.push(a[j] + b[j] * (i + 1));
        }

        // イベント後の得点の種類数を計算
        counts[i + 1] = count_unique_values(&next_scores);

        // イベント後の得点
        a = next_scores;
    }

    // 出力
    for count in counts {
        println!("{}", count);
    }
}

// 配列内のユニークな値の数をカウント
fn count_unique_values(a: &[usize]) -> usize {
    let mut set = HashSet::new();
    for &x in a {
        set.insert(x);
    }
    set.len()
}

// 標準入力から1行読み込み、usize型に変換して返す
fn read_line() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

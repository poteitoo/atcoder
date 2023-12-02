use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    // 配列aの要素の値をインデックスとして、その要素の個数を値とする配列dpを作成する
    let mut dp = vec![0; 1_000_000 + 1];
    for aa in a.iter() {
        dp[*aa] += aa;
    }

    // dpの累積和を取る
    for i in 1..dp.len() {
        dp[i] += dp[i - 1];
    }

    let mut ans = Vec::new();
    for aa in a.iter() {
        ans.push(dp[1_000_000] - dp[*aa]);
    }

    // 整数配列ansをスペース区切りで出力する
    let ans: Vec<String> = ans.iter().map(|&n| n.to_string()).collect();
    println!("{}", ans.join(" "));
}

fn can_form_path(grid: &[&str]) -> bool {
    let target = "snuke";
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    fn dfs(
        grid: &[&str],
        visited: &mut Vec<Vec<bool>>,
        word: &str,
        row: usize,
        col: usize,
        goal_pos: (usize, usize),
    ) -> bool {
        if goal_pos.0 == row && goal_pos.1 == col {
            return true; // 全ての文字を通過できた
        }

        if row >= grid.len()
            || col >= grid[0].len()
            || visited[row][col]
            || grid[row].as_bytes()[col] as char != word.chars().next().unwrap()
        {
            return false; // 配列の範囲外または既に訪れたマスまたは現在の文字が目標と一致しない
        }

        visited[row][col] = true; // マスを訪れたとマークする

        // 上下左右の隣接するマスを探索
        let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dx, dy) in &adjacent {
            let new_row = (row as i32 + dx) as usize;
            let new_col = (col as i32 + dy) as usize;
            if dfs(grid, visited, &word[1..], new_row, new_col, goal_pos) {
                return true;
            }
        }

        visited[row][col] = false; // バックトラックしてマスを未訪問に戻す

        false
    }

    dfs(grid, &mut visited, target, 0, 0, (2, 1))
}

fn main() {
    let grid = ["sns", "euk"];
    let can_form = can_form_path(&grid);
    println!("Can form path: {}", can_form);
}

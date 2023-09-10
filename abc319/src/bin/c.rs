use proconio::input;

// fn count_duplicates_in_row_or_column(matrix: Vec<Vec<usize>>) -> usize {
//     let mut duplicates = 0;

//     // 同一の行に同じ数が含まれているか確認
//     for row in &matrix {
//         let mut seen = std::collections::HashSet::new();
//         for &num in row {
//             if !seen.insert(num) {
//                 duplicates += 1;
//             }
//         }
//     }

//     // 同一の列に同じ数が含まれているか確認
//     for col in 0..3 {
//         let mut seen = std::collections::HashSet::new();
//         for row in 0..3 {
//             if !seen.insert(matrix[row][col]) {
//                 duplicates += 1;
//             }
//         }
//     }

//     // 斜めの要素に同じ数が含まれているか確認
//     let mut seen_diag1 = std::collections::HashSet::new();
//     let mut seen_diag2 = std::collections::HashSet::new();
//     for i in 0..3 {
//         if !seen_diag1.insert(matrix[i][i]) {
//             duplicates += 1;
//         }
//         if !seen_diag2.insert(matrix[i][2 - i]) {
//             duplicates += 1;
//         }
//     }

//     duplicates
// }

fn probability_of_same_numbers(matrix: Vec<Vec<usize>>) -> f64 {
    let mut count = 0;
    let mut total = 0;

    for i in 0..3 {
        for j in 0..3 {
            let num = matrix[i][j];
            let mut current_count = 0;
            for x in 0..3 {
                for y in 0..3 {
                    if (x != i || y != j) && matrix[x][y] == num {
                        current_count += 1;
                    }
                }
            }
            count += current_count;
            total += 8; // 8 possible cells after choosing the first one
        }
    }

    count as f64 / total as f64
}

fn main() {
    input! {
        c: [[usize; 3];3]
    }
    let prod = probability_of_same_numbers(c);
    println!("{}", 1.0 - prod);

    // let dup = count_duplicates_in_row_or_column(c);
    // println!("{:?}", dup);
}

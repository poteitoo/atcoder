use proconio::input;

fn main() {
    input! {
        n: usize
    }
    if n == 1 {
        println!("0");
        return;
    }
    let result = convert_to_custom_base5(n - 1);
    println!("{}", result);
}

fn convert_to_custom_base5(mut num: usize) -> String {
    let mut result = String::new();
    while num > 0 {
        let digit = num % 5;
        result.push_str(
            &match digit {
                0 => "0",
                1 => "2",
                2 => "4",
                3 => "6",
                4 => "8",
                _ => unreachable!(),
            }
            .to_string(),
        );
        num /= 5;
    }

    result.chars().rev().collect()
}

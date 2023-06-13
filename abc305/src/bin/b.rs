use proconio::input;

fn main() {
    input! {
        p: char,
        q: char
    }

    let lst = [0, 3, 4, 8, 9, 14, 23];
    let mut p = mapper(p);
    let mut q = mapper(q);
    if q > p {
        let temp = p;
        p = q;
        q = temp;
    }
    let res = lst[p] - lst[q];

    println!("{}", res);
}

fn mapper(c: char) -> usize {
    return if c == 'A' {
        0
    } else if c == 'B' {
        1
    } else if c == 'C' {
        2
    } else if c == 'D' {
        3
    } else if c == 'E' {
        4
    } else if c == 'F' {
        5
    } else {
        6
    };
}

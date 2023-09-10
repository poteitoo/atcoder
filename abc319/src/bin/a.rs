use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s:  String
    }

    let d: HashMap<&str, usize> = HashMap::from([
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ]);

    let ans = d.get(&s[..]).unwrap();
    println!("{}", ans);
}

use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: i32,
        q: i32,
        a: [i32; n],
    }

    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, x) in a.iter().enumerate() {
        if !map.contains_key(x) {
            map.insert(*x, Vec::new());
        }
        map.get_mut(x).unwrap().push((i + 1) as i32);
    }

    for _ in 0..q {
        input! {
            x: i32,
            k: usize,
        }
        let k = k - 1;
        if map.contains_key(&x) {
            let answer = if k < map[&x].len() { map[&x][k] } else { -1 };
            println!("{}", answer);
        } else {
            println!("{}", -1);
        }
    }
}

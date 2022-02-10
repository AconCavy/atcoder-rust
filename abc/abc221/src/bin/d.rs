use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    }

    let mut days = vec![0];
    for (a, b) in &ab {
        days.push(*a);
        days.push(*a + *b);
    }

    days.sort();
    days.dedup();
    let mut map: HashMap<i64, usize> = HashMap::new();
    for (i, d) in days.iter().enumerate() {
        map.insert(*d, i);
    }

    let mut cum = vec![0i32; map.len()];
    for (a, b) in &ab {
        let l = map[a];
        let r = map[&(a + b)];
        cum[l] += 1;
        cum[r] -= 1;
    }

    for i in 1..cum.len() {
        cum[i] += cum[i - 1];
    }

    let mut answer = vec![0i64; n + 1];
    for i in 0..(cum.len() - 1) {
        let d = days[i + 1] - days[i];
        answer[cum[i] as usize] += d;
    }

    println!("{}", answer.into_iter().skip(1).join(" "));
}

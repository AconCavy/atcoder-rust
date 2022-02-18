use proconio::*;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
    }

    let mut contests = HashSet::new();
    contests.insert("ABC".to_string());
    contests.insert("ARC".to_string());
    contests.insert("AGC".to_string());
    contests.insert("AHC".to_string());

    contests.remove(&s1);
    contests.remove(&s2);
    contests.remove(&s3);

    for x in contests {
        println!("{}", x);
    }
}

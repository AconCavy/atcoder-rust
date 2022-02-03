use itertools::Itertools;
use proconio::*;

#[fastout]
fn main() {
    input! {
        x:String
    }

    let y = x
        .split('.')
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
    let answer = y[0] + if y[1] / 100 >= 5 { 1 } else { 0 };
    println!("{}", answer);
}

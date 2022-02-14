use proconio::*;

#[fastout]
fn main() {
    input! {
        x: i32,
    }

    let answer = match x {
        0...39 => (40 - x).to_string(),
        40...69 => (70 - x).to_string(),
        70...89 => (90 - x).to_string(),
        _ => "expert".to_string(),
    };

    println!("{}", answer);
}

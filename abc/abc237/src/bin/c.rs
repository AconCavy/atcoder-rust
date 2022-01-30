use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut l = 0;
    let mut r = s.len() as i32 - 1;
    while r >= 0 && *s.get(r as usize).unwrap() == 'a' {
        r -= 1;
    }

    while l < r && *s.get(l as usize).unwrap() == 'a' {
        l += 1;
    }

    let lc = l;
    let rc = s.len() as i32 - r - 1;
    if lc > rc {
        println!("No");
        return;
    }

    let t = s[l as usize..(r as usize + 1)].iter().collect::<String>();
    let rt = t.chars().rev().collect::<String>();
    let answer = t == rt;
    println!("{}", if answer { "Yes" } else { "No" });
}

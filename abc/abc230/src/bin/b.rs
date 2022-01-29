use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s:Bytes,
    }

    let mut answer = false;
    let t = vec![b"oxx", b"xox", b"xxo"];
    for t in t {
        let mut ok = true;
        for (i, c) in s.iter().enumerate() {
            ok &= *c == t[i % 3];
        }

        answer |= ok;
    }

    println!("{}", if answer { "Yes" } else { "No" });
}

use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i32; n],
    }

    a.sort();
    for _ in 0..q {
        input! {
            x: i32,
        }

        let answer = n - lower_bound(&a, x);
        println!("{}", answer);
    }
}

fn lower_bound(source: &[i32], key: i32) -> usize {
    let mut l = -1 as i32;
    let mut r = source.len() as i32;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if source[m as usize] >= key {
            r = m;
        } else {
            l = m;
        }
    }

    r as usize
}

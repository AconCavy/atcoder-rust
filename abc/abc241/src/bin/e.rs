use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut answer = 0i64;
    let mut steps = Vec::new();
    let mut dict = HashMap::new();
    for i in 0..k {
        let xn = (answer % n as i64) as usize;
        if dict.contains_key(&xn) {
            let no_loop = *dict.get(&xn).unwrap();
            let loop_len = i - no_loop;
            let loop_count = (k - no_loop) / loop_len;
            let loop_mod = if k - no_loop < 0 {
                (k - no_loop + loop_len) % loop_len
            } else {
                (k - no_loop) % loop_len
            };

            answer = (answer - steps[no_loop as usize]) * loop_count
                + steps[(no_loop + loop_mod) as usize];
            break;
        }

        dict.insert(xn, i);
        steps.push(answer);
        answer += a[xn];
    }

    println!("{}", answer);
}

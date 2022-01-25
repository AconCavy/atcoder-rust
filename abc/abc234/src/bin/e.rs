use im_rc::HashSet;
use proconio::*;

#[fastout]
fn main() {
    input! {
        x: i64,
    }

    let mut hashset: HashSet<i64> = HashSet::new();

    for b in 0..10 as i64 {
        hashset.insert(b);
        for d in -9..10 {
            let mut y = b;
            if (d, b) == (0, 0) {
                continue;
            }

            for _ in 0..17 {
                let z = y % 10;
                if z + d < 0 || z + d >= 10 {
                    break;
                }
                y = y * 10 + z + d;
                hashset.insert(y);
            }
        }
    }

    let mut answer = 1e18 as i64;
    for v in hashset {
        if v >= x {
            answer = std::cmp::min(answer, v);
        }
    }

    println!("{}", answer);
}

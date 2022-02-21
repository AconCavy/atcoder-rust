use proconio::*;

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32),
        (c, d): (i32, i32),
    }

    let primes = sieve(200);
    for x in a..=b {
        let mut exist = false;
        for p in &primes {
            let y = p - x;
            exist |= c <= y && y <= d;
        }

        if !exist {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
}

fn sieve(upper: i32) -> Vec<i32> {
    let mut result = Vec::new();
    if upper < 2 {
        return result;
    }

    result.push(2);
    let mut sieve = vec![true; (upper + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 3;
    loop {
        if i as i32 > upper {
            break;
        }

        if sieve[i] {
            result.push(i as i32);
            let mut j = i + i;
            loop {
                if j < sieve.len() {
                    sieve[j] = false;
                } else {
                    break;
                }
                j += i;
            }
        }

        i += 2;
    }

    result
}

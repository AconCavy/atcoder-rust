use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize
    }

    let ng = n + 10;
    let mut front = vec![ng; n];
    let mut back = vec![ng; n];

    for i in 0..q {
        input! {
            t: i32
        }

        if t == 1 {
            input! {
                x: Usize1,
                y: Usize1
            }

            back[x] = y;
            front[y] = x;
        } else if t == 2 {
            input! {
                x: Usize1,
                y: Usize1
            }
            back[x] = ng;
            front[y] = ng;
        } else {
            input! {
                x: Usize1,
            }

            let mut answer: Vec<usize> = Vec::new();
            let mut curr = x;
            while front[curr] != ng {
                answer.push(front[curr]);
                curr = front[curr];
            }
            answer.reverse();
            answer.push(x);
            curr = x;
            while back[curr] != ng {
                answer.push(back[curr]);
                curr = back[curr];
            }

            println!(
                "{} {}",
                answer.len(),
                answer.into_iter().map(|x| x + 1).join(" ")
            );
        }
    }
}

use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let n2 = n * 2;
    let mut a: Vec<Vec<i32>> = Vec::new();
    for i in 0..n2 {
        input! {
            b: [i32; n2 - i - 1],
        }

        let mut c = vec![0; n2];
        for j in 0..(n2 - i - 1) {
            c[i + j + 1] = b[j];
        }
        a.push(c);
    }

    let mut answer = 0;
    let mut used = vec![false; n2];

    dfs(&mut answer, &a, &mut used, n2, 0, 0);
    println!("{}", answer);
}

fn dfs(
    answer: &mut i32,
    a: &Vec<Vec<i32>>,
    used: &mut Vec<bool>,
    end: usize,
    idx: usize,
    xor: i32,
) {
    if idx >= end {
        *answer = std::cmp::max(*answer, xor);
        return;
    }

    if used[idx] {
        dfs(answer, a, used, end, idx + 1, xor);
    } else {
        for i in (idx + 1)..end {
            if !used[i] {
                used[idx] = true;
                used[i] = true;
                dfs(answer, a, used, end, idx + 1, xor ^ a[idx][i]);
                used[idx] = false;
                used[i] = false;
            }
        }
    }
}

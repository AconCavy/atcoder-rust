use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![HashSet::new(); n];
    let mut map = HashMap::new();
    for i in 0..m {
        input! {
            s: Usize1,
            t: Usize1,
        }

        g[s].insert(t);
        map.insert((s, t), i);
    }

    let invalid = -1;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut dp = vec![invalid; n];
    dp[0] = 0;
    let mut prev = vec![n; n];

    while let Some(u) = queue.pop_front() {
        for v in &g[u] {
            if dp[*v] == invalid {
                dp[*v] = dp[u] + 1;
                prev[*v] = u;
                queue.push_back(*v);
            }
        }
    }

    if dp[n - 1] == -1 {
        println!("{}", vec![-1; m].into_iter().join("\n"));
        return;
    }

    let mut answer = vec![dp[n - 1]; m];

    let mut curr = n - 1;
    while curr != 0 {
        let s = prev[curr];
        let t = curr;
        g[s].remove(&t);

        queue.push_back(0);
        dp = vec![invalid; n];
        dp[0] = 0;
        while let Some(u) = queue.pop_front() {
            for v in &g[u] {
                if dp[*v] == invalid {
                    dp[*v] = dp[u] + 1;
                    queue.push_back(*v);
                }
            }
        }

        answer[map[&(s, t)]] = dp[n - 1];

        g[s].insert(t);
        curr = s;
    }

    println!("{}", answer.into_iter().join("\n"));
}

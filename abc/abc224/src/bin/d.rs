use proconio::marker::Usize1;
use proconio::*;
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        m: usize,
    }

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); 9];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }

        g[u].push(v);
        g[v].push(u);
    }

    input! {
        mut p: [Usize1; 8 as usize]
    }

    let mut set: HashSet<Vec<usize>> = HashSet::new();
    let mut queue: VecDeque<(Vec<usize>, i32)> = VecDeque::new();
    let empty = (8 * 9 / 2) as usize - p.iter().sum::<usize>();
    p.push(empty);
    set.insert(p.clone());
    queue.push_back((p, 0));
    let goal = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

    while let Some((mut p, c)) = queue.pop_front() {
        if p == goal {
            println!("{}", c);
            return;
        }

        let empty = p[8];
        for &v in &g[empty] {
            let target = p.iter().position(|&x| x == v).unwrap();
            p.swap(target, 8);
            if !set.contains(&p) {
                set.insert(p.clone());
                queue.push_back((p.clone(), c + 1));
            }
            p.swap(target, 8);
        }
    }

    println!("-1");
}

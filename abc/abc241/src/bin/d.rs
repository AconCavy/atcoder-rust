use itertools::Itertools;
use num_traits::abs;
use proconio::*;
use std::collections::{HashMap, HashSet};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut query = Vec::with_capacity(q);
    let mut v = Vec::with_capacity(q);
    for _ in 0..q {
        input! {
            t: usize,
            x: i64,
        }

        v.push(x);

        if t == 1 {
            query.push((t, x, -1));
        } else {
            input! {
                k: i64,
            }
            query.push((t, x, k));
        }
    }

    let (map, remap) = compress(&v);
    let n = map.len();
    let mut ft = FenwickTree::new(n);
    let f = |ft: &FenwickTree, l, r, k| ft.sum_of(l, r + 1) >= k;

    for (t, x, k) in query {
        if t == 1 {
            ft.add(map[&x], 1);
        } else if t == 2 {
            let r = map[&x];
            let mut ng = r + 1;
            let mut ok = 0;
            while abs(ok as i64 - ng as i64) > 1 {
                let m = (ok + ng) / 2;
                if f(&ft, m, r, k) {
                    ok = m;
                } else {
                    ng = m;
                }
            }

            let answer = if f(&ft, ok, r, k) { remap[&ok] } else { -1 };
            println!("{}", answer);
        } else {
            let l = map[&x];
            let mut ng = l - 1;
            let mut ok = n - 1;
            while abs(ok as i64 - ng as i64) > 1 {
                let m = (ok + ng) / 2;
                if f(&ft, l, m, k) {
                    ok = m;
                } else {
                    ng = m;
                }
            }

            let answer = if f(&ft, l, ok, k) { remap[&ok] } else { -1 };
            println!("{}", answer);
        }
    }
}

fn compress<T: std::cmp::Ord + std::hash::Hash + std::cmp::Eq + Clone>(
    source: &[T],
) -> (HashMap<T, usize>, HashMap<usize, T>) {
    let set: HashSet<_> = source.iter().collect();
    let mut map: HashMap<T, usize> = HashMap::new();
    let mut remap: HashMap<usize, T> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        map.insert(x.clone(), i);
        remap.insert(i, x.clone());
    }

    (map, remap)
}

struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(len: usize) -> Self {
        Self {
            len,
            data: vec![0; len],
        }
    }

    fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while i as usize <= self.len {
            self.data[(i - 1) as usize] += v;
            i += i & -i;
        }
    }

    fn sum(&self, len: usize) -> i64 {
        assert!(len <= self.len);
        let mut len = len as i32;
        let mut sum = 0i64;
        while len > 0 {
            sum += self.data[(len - 1) as usize];
            len -= len & -len;
        }

        sum
    }

    fn sum_of(&self, l: usize, r: usize) -> i64 {
        assert!(l < r && r <= self.len);
        self.sum(r) - self.sum(l)
    }

    fn bound(self, v: i64, compare: fn(i64, i64) -> bool) -> usize {
        if v >= self.data[0] {
            return 0;
        }

        let mut v = v;
        let mut x = 0;
        let mut k = 1;
        while k < self.len {
            k <<= 1;
        }

        let mut k = k;
        loop {
            if k <= 0 {
                break;
            }

            if x + k - 1 >= self.len || compare(v, self.data[x + k - 1]) {
                continue;
            }

            v -= self.data[x + k - 1];
            x += k;
            k >>= 1;
        }

        x
    }
}

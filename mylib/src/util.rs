#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

fn binary_search<F: Fn(i64) -> bool>(ng: i64, ok: i64, f: F) -> i64 {
    let mut ng = ng;
    let mut ok = ok;
    while (ok - ng).abs() > 1 {
        let m = (ok + ng) / 2;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}

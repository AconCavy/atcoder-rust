use im_rc::HashSet;
use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    // a[i] > b[i] && a[j] < b[j]

    let ca = compress(&a);
    let cb = compress(&b);
    let mut dict: HashMap<(usize, usize), i64> = HashMap::new();
    for ab in a.iter().zip(b).map(|(a, b)| (ca[&a], cb[&b])) {
        *dict.entry(ab).or_insert(0) += 1;
    }

    let mut abc: Vec<(usize, usize, i64)> = dict.into_iter().map(|((a, b), c)| (a, b, c)).collect();
    abc.sort_by_key(|x| (-(x.0 as i32), x.1));

    let mut answer = 0i64;
    let mut ft = FenwickTree::new(n);
    for (_, b, c) in abc {
        ft.add(b, c);
        answer += ft.sum(b + 1) * c;
    }

    println!("{}", answer);
}

fn compress(source: &[i64]) -> HashMap<i64, usize> {
    let set: HashSet<&i64> = source.iter().collect();
    let mut result: HashMap<i64, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }

    result
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
}

#![allow(dead_code)]

struct FenwickTree<T> {
    len: usize,
    data: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign> FenwickTree<T> {
    fn new(len: usize, e: T) -> Self {
        Self {
            len,
            data: vec![e.clone(); len],
            e,
        }
    }

    fn add(&mut self, i: usize, v: T) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while i as usize <= self.len {
            self.data[(i - 1) as usize] += v.clone();
            i += i & -i;
        }
    }

    fn accum(&self, len: usize) -> T {
        assert!(len <= self.len);
        let mut len = len as i32;
        let mut sum = self.e.clone();
        while len > 0 {
            sum += self.data[(len - 1) as usize].clone();
            len -= len & -len;
        }

        sum
    }

    fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        assert!(l <= r && r <= self.len);
        self.accum(r) - self.accum(l)
    }

    fn bound<F: Fn(T, T) -> bool>(&self, v: T, compare: F) -> usize
    where
        T: std::ops::SubAssign,
    {
        if compare(v.clone(), self.data[0].clone()) {
            return 0;
        }

        let mut v = v.clone();
        let mut x = 0;
        let mut k = 1;
        while k < self.len {
            k <<= 1;
        }

        while k > 0 {
            if x + k - 1 >= self.len || compare(v.clone(), self.data[x + k - 1].clone()) {
                k >>= 1;
                continue;
            }

            v -= self.data[x + k - 1].clone();
            x += k;
            k >>= 1;
        }

        x
    }
}

#[test]
fn fenwick_tree_naive_test() {
    for n in 0..=50 {
        let e = 0;
        let mut ft = FenwickTree::new(n, e);
        assert_eq!(ft.len, n);

        for i in 0..n {
            ft.add(i, (i * i) as i32);
        }

        for i in 0..=n {
            let mut sum = e;
            for j in 0..i {
                sum += (j * j) as i32;
            }
            assert_eq!(ft.accum(i), sum);
        }

        for l in 0..=n {
            for r in l..=n {
                let mut sum = e;
                for i in l..r {
                    sum += (i * i) as i32;
                }
                assert_eq!(ft.sum(l, r), sum);
            }
        }
    }
}

#[test]
fn fenwick_tree_bound_test() {
    let n = 10;
    let e = 0;
    let mut ft = FenwickTree::new(n, e);
    for i in 0..n {
        ft.add(i, (i + 1) as i32);
    }
    let mut cum = vec![e; n + 1];
    for i in 1..=n {
        cum[i] = cum[i - 1] + i as i32;
    }

    let lower_bound = |x, y| x <= y;
    let upper_bound = |x, y| x < y;

    for i in 0..n {
        assert_eq!(ft.bound(cum[i + 1], lower_bound), i);
        assert_eq!(ft.bound(cum[i + 1] + 1, lower_bound), (i + 1).min(n));

        assert_eq!(ft.bound(cum[i + 1], upper_bound), (i + 1).min(n));
        assert_eq!(ft.bound(cum[i + 1] - 1, upper_bound), i);
    }
}

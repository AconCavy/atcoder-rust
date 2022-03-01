#![allow(dead_code)]

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

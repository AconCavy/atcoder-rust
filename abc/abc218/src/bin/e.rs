use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut dsu = Dsu::new(n);
    let mut edges = Vec::new();
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: i64,
        }

        if c < 0 {
            dsu.merge(a, b);
        } else {
            edges.push((a, b, c));
        }
    }

    edges.sort_by_key(|x| x.2);
    let mut answer = 0;
    for (a, b, c) in edges {
        if dsu.same(a, b) {
            answer += c;
        } else {
            dsu.merge(a, b);
        }
    }

    println!("{}", answer);
}

struct Dsu {
    len: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            len: size,
            parent_or_size: vec![-1; size],
        }
    }

    fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.len);
        assert!(v < self.len);
        let (mut x, mut y) = (self.leader_of(u), self.leader_of(v));
        if x == y {
            return x;
        }

        if -self.parent_or_size[u] < -self.parent_or_size[v] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.len);
        assert!(v < self.len);
        self.leader_of(u) == self.leader_of(v)
    }

    fn leader_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        if self.parent_or_size[u] < 0 {
            return u;
        }

        self.parent_or_size[u] = self.leader_of(self.parent_or_size[u] as usize) as i32;
        self.parent_or_size[u] as usize
    }

    fn size_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        let u = self.leader_of(u);
        -self.parent_or_size[u] as usize
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = vec![Vec::new(); self.len];
        for i in 0..self.len {
            result.get_mut(self.leader_of(i)).unwrap().push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}

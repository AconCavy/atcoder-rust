use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        q: i32,
    }

    let mut es: Vec<Edge> = Vec::new();
    for _ in 0..m {
        input! {
            a: i32,
            b: i32,
            c: i32,
        }

        es.push(Edge::new(-1, a - 1, b - 1, c));
    }

    for i in 0..q {
        input! {
            a: i32,
            b: i32,
            c: i32,
        }

        es.push(Edge::new(i, a - 1, b - 1, c));
    }

    es.sort_by(|a, b| a.c.cmp(&b.c));
    let mut dsu = Dsu::new(n as usize);
    let mut answers = vec![false; q as usize];

    for e in es {
        if dsu.same(e.u as usize, e.v as usize) {
            continue;
        }

        if e.id == -1 {
            dsu.merge(e.u as usize, e.v as usize);
        } else {
            answers[e.id as usize] = true;
        }
    }

    for answer in answers {
        println!("{}", if answer { "Yes" } else { "No" });
    }
}

struct Edge {
    id: i32,
    u: i32,
    v: i32,
    c: i32,
}

impl Edge {
    fn new(id: i32, u: i32, v: i32, c: i32) -> Self {
        Edge { id, u, v, c }
    }
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

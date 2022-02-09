use proconio::marker::Chars;
use proconio::*;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut a: Vec<Vec<char>> = vec![vec![' '; m]; n * 2];
    for i in 0..(n * 2) {
        input! {
            line: Chars
        }

        for (j, c) in line.into_iter().enumerate() {
            a[i][j] = c;
        }
    }

    let mut p = Vec::with_capacity(n * 2);
    for i in 0..(n * 2) {
        p.push(Player { id: i, win: 0 });
    }

    for i in 0..m {
        for j in 0..n {
            let p1 = j * 2;
            let p2 = j * 2 + 1;
            let a1 = a[p[p1].id][i];
            let a2 = a[p[p2].id][i];
            if rps(a1, a2) {
                p[p1].win += 1;
            } else if rps(a2, a1) {
                p[p2].win += 1;
            }
        }

        p.sort();
    }

    for p in p {
        println!("{}", p.id + 1);
    }
}

fn rps(p1: char, p2: char) -> bool {
    p1 == 'G' && p2 == 'C' || p1 == 'C' && p2 == 'P' || p1 == 'P' && p2 == 'G'
}

#[derive(Eq, PartialEq)]
struct Player {
    id: usize,
    win: i32,
}

impl PartialOrd<Self> for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .win
            .cmp(&self.win)
            .then_with(|| self.id.cmp(&other.id))
    }
}

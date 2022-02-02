use proconio::*;

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let n = 1 << 20;
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = i;
    }

    fn leader_of(p: &mut [usize], id: usize) -> usize {
        if p[id] == id {
            return id;
        }
        p[id] = leader_of(p, p[id]);
        p[id]
    }

    let mut a = vec![-1; n];
    for _ in 0..q {
        input! {
            t:usize,
            x:i64
        }

        let h = (x % n as i64) as usize;
        if t == 1 {
            let id = leader_of(&mut p, h);
            a[id] = x;
            p[id] = (id + 1) % n;
        } else {
            println!("{}", a[h]);
        }
    }
}

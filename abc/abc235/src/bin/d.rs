use proconio::*;
use std::collections::HashMap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        a: i64,
        n: i64,
    }

    let mut queue: VecDeque<i64> = VecDeque::new();
    let mut map: HashMap<i64, i64> = HashMap::new();
    queue.push_back(1);
    map.insert(1, 0);

    let mut limit = 1;
    while n / limit > 0 {
        limit *= 10;
    }

    let push =
        |map: &mut HashMap<i64, i64>, queue: &mut VecDeque<i64>, limit: i64, x: i64, count: i64| {
            if !map.contains_key(&x) && x <= limit {
                map.insert(x, count);
                queue.push_back(x);
            }
        };

    let shift = |v: i64| -> i64 {
        let mut ten = 1;
        while v / ten >= 10 {
            ten *= 10;
        }

        (v % 10 * ten) + (v / 10)
    };

    while let Some(x) = queue.pop_front() {
        let next = map[&x] + 1;
        push(&mut map, &mut queue, limit, x * a, next);
        if x % 10 != 0 {
            push(&mut map, &mut queue, limit, shift(x), next);
        }
    }

    let answer = if map.contains_key(&n) { map[&n] } else { -1 };
    println!("{}", answer);
}

#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufReader, StdinLock};
use std::mem::*;

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::source::line::LineSource;
use proconio::*;
use rand::rngs::StdRng;
use rand::Rng;

fn main() {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    const N: usize = 100;
    const DIR: [Direction; 4] = [
        Direction::Front,
        Direction::Back,
        Direction::Left,
        Direction::Right,
    ];
    const SEED: u64 = 0;

    input! {
        from &mut source,
        _f: [u8; N],
    }

    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(SEED);
    for _ in 0..N {
        input! {
            from &mut source,
            _p: usize,
        }
        println!("{}", DIR[rng.gen_range(0, 4)]);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Front,
    Back,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Front => 'F',
            Direction::Back => 'B',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{}", c)
    }
}

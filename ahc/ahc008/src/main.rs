#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::BufReader;

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use proconio::*;
use rand::{thread_rng, Rng};

const MAP_SIZE: usize = 30 + 2;
const INVALID: i32 = -1;

fn main() {
    let stdin = io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
    }

    let mut pets = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            from &mut source,
            px: i32,
            py: i32,
            _pt: usize,
        }

        pets.push(Pet::new(px, py));
    }

    input! {
        from &mut source,
        m: usize,
    }

    let mut humans = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            from &mut source,
            hx: i32,
            hy: i32,
        }

        humans.push(Human::new(hx, hy));
    }

    let mut blocks = vec![vec![false; MAP_SIZE]; MAP_SIZE];

    for i in 0..MAP_SIZE {
        blocks[0][i] = true;
        blocks[MAP_SIZE - 1][i] = true;
        blocks[i][0] = true;
        blocks[i][MAP_SIZE - 1] = true;
    }

    for _ in 0..300 {
        let mut exists = vec![vec![false; MAP_SIZE]; MAP_SIZE];
        for pos in humans.iter().map(|x| x.position) {
            exists[pos.x as usize][pos.y as usize] = true;
        }

        for pos in pets.iter().map(|x| x.position) {
            exists[pos.x as usize][pos.y as usize] = true;
        }

        let mut answer = vec!['.'; m];
        for i in 0..m {
            answer[i] = humans[i].act(&pets, &exists, &mut blocks);
        }

        println!("{}", answer.into_iter().join(""));

        input! {
            from &mut source,
            actions: [Chars; n]
        }

        for (i, action) in actions.into_iter().enumerate() {
            pets[i].act(
                &action
                    .into_iter()
                    .map(|c| Direction::from_char(c))
                    .collect_vec(),
            );
        }
    }
}

fn calc_steps(blocks: &[Vec<bool>], (sx, sy): (i32, i32)) -> (Vec<Vec<i32>>, Vec<Vec<(i32, i32)>>) {
    let mut steps = vec![vec![INVALID; MAP_SIZE]; MAP_SIZE];
    let mut steps_from = vec![vec![(INVALID, INVALID); MAP_SIZE]; MAP_SIZE];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((sx, sy));
    steps[sx as usize][sy as usize] = 0;
    while let Some((cx, cy)) = queue.pop_front() {
        for (dx, dy) in Direction::DIRECTION4.iter().map(|x| x.to_tuple()) {
            if cx + dx < 0 || cy + dy < 0 {
                continue;
            }
            let nx = (cx + dx) as usize;
            let ny = (cy + dy) as usize;
            if MAP_SIZE <= nx || MAP_SIZE <= ny || blocks[nx][ny] || steps[nx][ny] != INVALID {
                continue;
            }

            steps[nx][ny] = steps[cx as usize][cy as usize] + 1;
            steps_from[nx][ny] = (cx, cy);
            queue.push_back((nx as i32, ny as i32));
        }
    }

    (steps, steps_from)
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const DIRECTION4: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    fn inv(&self) -> Direction {
        match self {
            Direction::None => Direction::None,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::None => '.',
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }

    fn from_tuple(v: (i32, i32)) -> Direction {
        match v {
            (-1, 0) => Direction::Up,
            (1, 0) => Direction::Down,
            (0, -1) => Direction::Left,
            (0, 1) => Direction::Right,
            (_, _) => Direction::None,
        }
    }

    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn distance(&self, other: &Vector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn translate_by_direction(&mut self, dir: &Direction) {
        let (x, y) = dir.to_tuple();
        self.translate(x, y);
    }

    fn neighbor(&self, dir: &Direction) -> Self {
        let (x, y) = dir.to_tuple();
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn is_valid(&self) -> bool {
        0 <= self.x && self.x < MAP_SIZE as i32 && 0 <= self.y && self.y < MAP_SIZE as i32
    }
}

#[derive(Debug, Clone)]
struct Pet {
    position: Vector,
}

impl Pet {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Vector { x, y },
        }
    }

    fn act(&mut self, action: &[Direction]) {
        for dir in action {
            self.position.translate_by_direction(dir);
        }
    }
}

#[derive(Debug, Clone)]
struct Human {
    position: Vector,
}

impl Human {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Vector { x, y },
        }
    }

    fn act(&mut self, pets: &[Pet], exists: &[Vec<bool>], blocks: &mut [Vec<bool>]) -> char {
        let (steps, steps_from) = calc_steps(blocks, (self.position.x, self.position.y));

        let mut target_step = 1e9 as i32;
        let mut target = INVALID;
        for (i, pet) in pets.iter().enumerate() {
            let step = steps[pet.position.x as usize][pet.position.y as usize];
            if step == INVALID {
                continue;
            }

            if step < target_step {
                target_step = step;
                target = i as i32;
            }
        }

        if target == INVALID {
            return '.';
        }

        let target = &pets[target as usize];
        let mut cx = target.position.x;
        let mut cy = target.position.y;
        loop {
            if steps_from[cx as usize][cy as usize] == (INVALID, INVALID) {
                break;
            }

            let tmp = steps_from[cx as usize][cy as usize];
            cx = tmp.0;
            cy = tmp.1;
        }

        let direction = Direction::from_tuple((cx - self.position.x, cy - self.position.y));

        if target_step < 3 {
            let direction = direction.inv();
            self.position.translate_by_direction(&direction);
            return direction.to_char();
        } else if 3 <= target_step && target_step < 6 {
            if !exists[cx as usize][cy as usize] {
                blocks[cx as usize][cy as usize] = true;
                return direction.to_char().to_ascii_lowercase();
            } else {
                loop {
                    let direction = match thread_rng().gen_range(0, 5) {
                        0 => Direction::Up,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        3 => Direction::Right,
                        _ => Direction::None,
                    };

                    let neighbor = self.position.neighbor(&direction);
                    if neighbor.is_valid() && !blocks[neighbor.x as usize][neighbor.y as usize] {
                        self.position.translate_by_direction(&direction);
                        return direction.to_char();
                    }
                }
            }
        } else {
            self.position.translate_by_direction(&direction);
            return direction.to_char();
        }
    }
}

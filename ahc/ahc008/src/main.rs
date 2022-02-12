#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::BufReader;

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use proconio::*;

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

    for i in 0..m {
        let mut d = 1e9 as i32;
        for j in 0..n {
            d = d.min(
                (&pets[j].position.x - &humans[i].position.x).abs()
                    + (&pets[j].position.y - &humans[i].position.y).abs(),
            );
        }

        let x1 = (&humans[i].position.x - d).max(1).min(30);
        let y1 = (&humans[i].position.y - d).max(1).min(30);
        let x2 = (&humans[i].position.x + d).max(1).min(30);
        let y2 = (&humans[i].position.y + d).max(1).min(30);
        humans[i].area = Area::new(x1, y1, x2, y2);
    }

    for i in 0..m {
        let mut area = humans[i].area.clone();
        for j in 0..m {
            if area.intersect(&humans[j].area) {
                area = area.combine(&humans[j].area);
            }
        }

        humans[i].area = area;
    }

    for _ in 0..300 {
        let mut human_exists = vec![vec![false; MAP_SIZE]; MAP_SIZE];
        let mut pet_exists = vec![vec![false; MAP_SIZE]; MAP_SIZE];
        for pos in humans.iter().map(|x| x.position) {
            human_exists[pos.x as usize][pos.y as usize] = true;
        }

        for pos in pets.iter().map(|x| x.position) {
            pet_exists[pos.x as usize][pos.y as usize] = true;
        }

        let mut answer = vec!['.'; m];
        for i in 0..m {
            answer[i] = humans[i].act(&human_exists, &pet_exists, &mut blocks);
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

fn calc_distance(blocks: &[Vec<bool>], pets: &[Pet]) -> Vec<Vec<i32>> {
    let mut distance = vec![vec![INVALID; MAP_SIZE]; MAP_SIZE];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    for pos in pets.iter().map(|x| x.position) {
        distance[pos.x as usize][pos.y as usize] = 0;
        queue.push_back((pos.x, pos.y));
    }

    while let Some((cx, cy)) = queue.pop_front() {
        for (dx, dy) in Direction::DIRECTION4.iter().map(|x| x.to_tuple()) {
            if cx + dx < 0 || cy + dy < 0 {
                continue;
            }
            let nx = (cx + dx) as usize;
            let ny = (cy + dy) as usize;
            if MAP_SIZE <= nx || MAP_SIZE <= ny || blocks[nx][ny] || distance[nx][ny] != INVALID {
                continue;
            }

            distance[nx][ny] = distance[cx as usize][cy as usize] + 1;
            queue.push_back((nx as i32, ny as i32));
        }
    }

    distance
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

    fn inverse(&self) -> Direction {
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

    fn neighbor(&self, dir: &Direction) -> Vector {
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
    area: Area,
}

impl Human {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Vector { x, y },
            area: Area::new(0, 0, MAP_SIZE as i32, MAP_SIZE as i32),
        }
    }

    fn act(
        &mut self,
        human_exists: &[Vec<bool>],
        pet_exists: &[Vec<bool>],
        blocks: &mut [Vec<bool>],
    ) -> char {
        if self.is_up_limit() {
            // UL
            if self.is_left_limit() {
                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Left);
                if result != '.' {
                    return result;
                }

                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Up);
                if result != '.' {
                    return result;
                }

                let result = self.move_or_stay(blocks, &Direction::Right);
                if result != '.' {
                    return result;
                }
            }

            // UR
            if self.is_right_limit() {
                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Up);
                if result != '.' {
                    return result;
                }

                let result =
                    self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Right);
                if result != '.' {
                    return result;
                }

                let result = self.move_or_stay(blocks, &Direction::Down);
                if result != '.' {
                    return result;
                }
            }
        }

        if self.is_down_limit() {
            // DR
            if self.is_right_limit() {
                let result =
                    self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Right);
                if result != '.' {
                    return result;
                }

                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Down);
                if result != '.' {
                    return result;
                }

                let result = self.move_or_stay(blocks, &Direction::Left);
                if result != '.' {
                    return result;
                }
            }

            // DL
            if self.is_left_limit() {
                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Down);
                if result != '.' {
                    return result;
                }

                let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Left);
                if result != '.' {
                    return result;
                }

                let result = self.move_or_stay(blocks, &Direction::Up);
                if result != '.' {
                    return result;
                }
            }
        }

        // U
        if self.is_up_limit() {
            let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Up);
            if result != '.' {
                return result;
            }

            let result = self.move_or_stay(blocks, &Direction::Right);
            if result != '.' {
                return result;
            }
        }

        // R
        if self.is_right_limit() {
            let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Right);
            if result != '.' {
                return result;
            }

            let result = self.move_or_stay(blocks, &Direction::Down);
            if result != '.' {
                return result;
            }
        }

        // D
        if self.is_down_limit() {
            let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Down);
            if result != '.' {
                return result;
            }

            let result = self.move_or_stay(blocks, &Direction::Left);
            if result != '.' {
                return result;
            }
        }

        // L
        if self.is_left_limit() {
            let result = self.block_or_stay(human_exists, pet_exists, blocks, &Direction::Left);
            if result != '.' {
                return result;
            }

            let result = self.move_or_stay(blocks, &Direction::Up);
            if result != '.' {
                return result;
            }
        }

        self.move_or_stay(blocks, &Direction::Up)
    }

    fn move_or_stay(&mut self, blocks: &mut [Vec<bool>], direction: &Direction) -> char {
        let p = self.position.neighbor(&direction);
        return if !blocks[p.x as usize][p.y as usize] {
            self.position = p;
            direction.to_char()
        } else {
            '.'
        };
    }

    fn block_or_stay(
        &self,
        human_exists: &[Vec<bool>],
        pet_exists: &[Vec<bool>],
        blocks: &mut [Vec<bool>],
        direction: &Direction,
    ) -> char {
        let is_blockable =
            |human_exists: &[Vec<bool>], pet_exists: &[Vec<bool>], position: &Vector| -> bool {
                if human_exists[position.x as usize][position.y as usize]
                    || pet_exists[position.x as usize][position.y as usize]
                {
                    return false;
                }

                for dir in &Direction::DIRECTION4 {
                    let p = position.neighbor(&dir);
                    if p.is_valid() && pet_exists[p.x as usize][p.y as usize] {
                        return false;
                    }
                }

                true
            };

        let p = self.position.neighbor(&direction);
        return if !blocks[p.x as usize][p.y as usize] && is_blockable(human_exists, pet_exists, &p)
        {
            blocks[p.x as usize][p.y as usize] = true;
            direction.to_char().to_ascii_lowercase()
        } else {
            '.'
        };
    }

    fn is_up_limit(&self) -> bool {
        self.area.is_up_edge(&self.position)
    }

    fn is_down_limit(&self) -> bool {
        self.area.is_down_edge(&self.position)
    }

    fn is_left_limit(&self) -> bool {
        self.area.is_left_edge(&self.position)
    }

    fn is_right_limit(&self) -> bool {
        self.area.is_right_edge(&self.position)
    }
}

#[derive(Debug, Clone)]
struct Area {
    position1: Vector,
    position2: Vector,
}

impl Area {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            position1: Vector { x: x1, y: y1 },
            position2: Vector { x: x2, y: y2 },
        }
    }

    fn combine(&self, other: &Area) -> Self {
        Self {
            position1: Vector {
                x: self.position1.x.min(other.position1.x),
                y: self.position1.y.min(other.position1.y),
            },
            position2: Vector {
                x: self.position2.x.max(other.position2.x),
                y: self.position2.y.max(other.position2.y),
            },
        }
    }

    fn intersect(&self, other: &Area) -> bool {
        self.position1.x <= other.position2.x
            && other.position1.x <= self.position2.x
            && self.position1.y <= other.position2.y
            && other.position1.y <= self.position2.y
    }

    fn is_up_edge(&self, position: &Vector) -> bool {
        position.x == self.position1.x
    }

    fn is_down_edge(&self, position: &Vector) -> bool {
        position.x == self.position2.x
    }

    fn is_left_edge(&self, position: &Vector) -> bool {
        position.y == self.position1.y
    }

    fn is_right_edge(&self, position: &Vector) -> bool {
        position.y == self.position2.y
    }
}

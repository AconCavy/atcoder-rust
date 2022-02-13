#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::BufReader;

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use proconio::*;

const MAP_SIZE: usize = 30 + 2;

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

    let mut grid = Grid::new(MAP_SIZE);
    grid.init();

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
        grid.update_human_exists(&humans);
        grid.update_pet_exists(&pets);

        let mut answer = vec!['.'; m];
        for i in 0..m {
            answer[i] = humans[i].act(&mut grid);
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

    fn to_vector(&self) -> Vector {
        match self {
            Direction::None => Vector::new(0, 0),
            Direction::Up => Vector::new(-1, 0),
            Direction::Down => Vector::new(1, 0),
            Direction::Left => Vector::new(0, -1),
            Direction::Right => Vector::new(0, 1),
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

    fn from_vector(v: Vector) -> Direction {
        match v {
            Vector { x: -1, y: 0 } => Direction::Up,
            Vector { x: 1, y: 0 } => Direction::Down,
            Vector { x: 0, y: -1 } => Direction::Left,
            Vector { x: 0, y: 1 } => Direction::Right,
            _ => Direction::None,
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
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn translate(&mut self, v: &Vector) {
        self.x += v.x;
        self.y += v.y;
    }

    fn neighbor(&self, direction: &Direction) -> Vector {
        let v = direction.to_vector();
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
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
        for direction in action {
            self.position.translate(&direction.to_vector());
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

    fn act(&mut self, grid: &mut Grid) -> char {
        let move_char = |direction: &Direction| direction.to_char();
        let block_char = |direction: &Direction| direction.to_char().to_ascii_lowercase();

        // UL
        if self.is_up_limit() && self.is_left_limit() {
            let direction = Direction::Left;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Up;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Right;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // UR
        if self.is_up_limit() && self.is_right_limit() {
            let direction = Direction::Up;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Right;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Down;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // DR
        if self.is_down_limit() && self.is_right_limit() {
            let direction = Direction::Right;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Down;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Left;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // DL
        if self.is_down_limit() && self.is_left_limit() {
            let direction = Direction::Down;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Left;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Up;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // U
        if self.is_up_limit() {
            let direction = Direction::Up;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Right;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // R
        if self.is_right_limit() {
            let direction = Direction::Right;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Down;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // D
        if self.is_down_limit() {
            let direction = Direction::Down;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Left;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        // L
        if self.is_left_limit() {
            let direction = Direction::Left;
            if self.try_block(grid, &direction) {
                return block_char(&direction);
            }

            let direction = Direction::Up;
            if self.try_move(grid, &direction) {
                return move_char(&direction);
            }
        }

        let direction = Direction::Up;
        return if self.try_move(grid, &direction) {
            move_char(&direction)
        } else {
            '.'
        };
    }

    fn try_move(&mut self, grid: &mut Grid, direction: &Direction) -> bool {
        let position = self.position.neighbor(&direction);
        return if grid.is_movable(&position) {
            self.position = position;
            true
        } else {
            false
        };
    }

    fn try_block(&self, grid: &mut Grid, direction: &Direction) -> bool {
        let position = self.position.neighbor(&direction);
        return if grid.is_blockable(&position) {
            grid.block(&position);
            true
        } else {
            false
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
            position1: Vector::new(x1, y1),
            position2: Vector::new(x2, y2),
        }
    }

    fn combine(&self, other: &Area) -> Self {
        Self {
            position1: Vector::new(
                self.position1.x.min(other.position1.x),
                self.position1.y.min(other.position1.y),
            ),
            position2: Vector::new(
                self.position2.x.max(other.position2.x),
                self.position2.y.max(other.position2.y),
            ),
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

struct Grid {
    size: usize,
    human_exists: Vec<Vec<bool>>,
    pet_exists: Vec<Vec<bool>>,
    block_exists: Vec<Vec<bool>>,
}

impl Grid {
    const INVALID: i32 = -1;

    fn new(size: usize) -> Self {
        Self {
            size,
            human_exists: vec![vec![false; size]; size],
            pet_exists: vec![vec![false; size]; size],
            block_exists: vec![vec![false; size]; size],
        }
    }

    fn init(&mut self) {
        for i in 0..self.size {
            self.block_exists[0][i] = true;
            self.block_exists[self.size - 1][i] = true;
            self.block_exists[i][0] = true;
            self.block_exists[i][self.size - 1] = true;
        }
    }

    fn update_human_exists(&mut self, humans: &[Human]) {
        self.human_exists = vec![vec![false; self.size]; self.size];
        for position in humans.iter().map(|x| x.position) {
            self.human_exists[position.x as usize][position.y as usize] = true;
        }
    }

    fn update_pet_exists(&mut self, pets: &[Pet]) {
        self.pet_exists = vec![vec![false; self.size]; self.size];
        for position in pets.iter().map(|x| x.position) {
            self.pet_exists[position.x as usize][position.y as usize] = true;
        }
    }

    fn calc_distance_from(&self, position: &Vector) -> (Vec<Vec<i32>>, Vec<Vec<Vector>>) {
        let mut distance = vec![vec![Self::INVALID; self.size]; self.size];
        let mut steps_from =
            vec![vec![Vector::new(Self::INVALID, Self::INVALID); self.size]; self.size];
        let mut queue: VecDeque<Vector> = VecDeque::new();
        queue.push_back(position.clone());
        distance[position.x as usize][position.y as usize] = 0;

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(x)) {
                if self.is_movable(&np) && distance[np.x as usize][np.y as usize] == Self::INVALID {
                    distance[np.x as usize][np.y as usize] =
                        distance[cp.x as usize][cp.y as usize] + 1;
                    steps_from[np.x as usize][np.y as usize] = cp.clone();
                    queue.push_back(np);
                }
            }
        }

        (distance, steps_from)
    }

    fn calc_distance(&self, pets: &[Pet]) -> Vec<Vec<i32>> {
        let mut distance = vec![vec![Self::INVALID; self.size]; self.size];
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in pets.iter().map(|x| x.position) {
            distance[position.x as usize][position.y as usize] = 0;
            queue.push_back(position.clone());
        }

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(x)) {
                if self.is_movable(&np) && distance[np.x as usize][np.y as usize] == Self::INVALID {
                    distance[np.x as usize][np.y as usize] =
                        distance[cp.x as usize][cp.y as usize] + 1;
                    queue.push_back(np);
                }
            }
        }

        distance
    }

    fn is_valid(&self, position: &Vector) -> bool {
        0 <= position.x
            && position.x < self.size as i32
            && 0 <= position.y
            && position.y < self.size as i32
    }

    fn is_movable(&self, position: &Vector) -> bool {
        self.is_valid(&position) && !self.block_exists[position.x as usize][position.y as usize]
    }

    fn is_blockable(&self, position: &Vector) -> bool {
        if !self.is_valid(&position)
            || self.block_exists[position.x as usize][position.y as usize]
            || self.human_exists[position.x as usize][position.y as usize]
            || self.pet_exists[position.x as usize][position.y as usize]
        {
            return false;
        }

        for np in Direction::DIRECTION4.iter().map(|x| position.neighbor(x)) {
            if self.is_valid(&np) && self.pet_exists[np.x as usize][np.y as usize] {
                return false;
            }
        }

        true
    }

    fn block(&mut self, position: &Vector) {
        if self.is_blockable(&position) {
            self.block_exists[position.x as usize][position.y as usize] = true;
        }
    }
}

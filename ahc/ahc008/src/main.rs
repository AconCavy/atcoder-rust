#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::BufReader;

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use proconio::*;
use rand::Rng;

const GRID_SIZE: usize = 30 + 2;
const GRID_MIN: i32 = 1;
const GRID_MAX: i32 = 30;

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
            pt: usize,
        }

        pets.push(Pet::new(
            px,
            py,
            match pt {
                1 => Kind::Cow,
                2 => Kind::Pig,
                3 => Kind::Rabbit,
                4 => Kind::Dog,
                5 => Kind::Cat,
                _ => Kind::None,
            },
        ));
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

    let mut grid = Grid::new(GRID_SIZE);
    grid.init();

    let threshold = (n - m).max(4).min(10) as i32;
    for _ in 0..300 {
        grid.update(&humans, &pets);

        let mut answer = vec!['.'; m];
        for (i, human) in humans.iter_mut().enumerate() {
            answer[i] = human.act(&mut grid, threshold);
        }

        println!("{}", answer.into_iter().join(""));

        input! {
            from &mut source,
            actions: [Chars; n]
        }

        for (pet, action) in pets.iter_mut().zip(actions.into_iter()) {
            pet.act(&action);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Kind {
    None,
    Cow,
    Pig,
    Rabbit,
    Dog,
    Cat,
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
    kind: Kind,
}

impl Pet {
    fn new(x: i32, y: i32, kind: Kind) -> Self {
        Self {
            position: Vector { x, y },
            kind,
        }
    }

    fn act(&mut self, action: &[char]) {
        for c in action {
            self.position
                .translate(&Direction::from_char(*c).to_vector());
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

    fn act(&mut self, grid: &mut Grid, threshold: i32) -> char {
        let distance = grid.distances[self.position.x as usize][self.position.y as usize];
        if distance == Grid::INVALID_I32 {
            return '.';
        }

        if distance > threshold {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.position.neighbor(&direction);
                if grid.is_valid(&np) && grid.distances[np.x as usize][np.y as usize] < distance {
                    if let Some(result) = self.try_move_to(grid, &direction) {
                        return result;
                    }
                }
            }
        } else if distance < 3 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.position.neighbor(&direction);
                if grid.is_valid(&np) && grid.distances[np.x as usize][np.y as usize] > distance {
                    if let Some(result) = self.try_move_to(grid, &direction) {
                        return result;
                    }
                }
            }
        } else {
            let blocked = Direction::DIRECTION4
                .iter()
                .map(|x| self.position.neighbor(x))
                .filter(|v| grid.block_exists[v.x as usize][v.y as usize])
                .count();

            if blocked < 2 {
                for direction in Direction::DIRECTION4.iter() {
                    let np = self.position.neighbor(&direction);
                    if grid.is_valid(&np) && grid.distances[np.x as usize][np.y as usize] < distance
                    {
                        if let Some(result) = self.try_block(grid, &direction) {
                            return result;
                        }
                    }
                }
            }
        }
        '.'
    }

    fn try_move_to(&mut self, grid: &mut Grid, direction: &Direction) -> Option<char> {
        let position = self.position.neighbor(&direction);
        return if grid.is_movable(&position) {
            grid.move_to(&position);
            self.position = position;
            Some(direction.to_char())
        } else {
            None
        };
    }

    fn try_block(&self, grid: &mut Grid, direction: &Direction) -> Option<char> {
        let position = self.position.neighbor(&direction);
        return if grid.is_blockable(&position) {
            grid.block(&position);
            Some(direction.to_char().to_ascii_lowercase())
        } else {
            None
        };
    }
}

struct Grid {
    size: usize,
    human_exists: Vec<Vec<bool>>,
    pet_exists: Vec<Vec<bool>>,
    block_exists: Vec<Vec<bool>>,
    distances: Vec<Vec<i32>>,
}

impl Grid {
    const INVALID_I32: i32 = -1;
    const INVALID_VECTOR: Vector = Vector {
        x: Self::INVALID_I32,
        y: Self::INVALID_I32,
    };

    fn new(size: usize) -> Self {
        Self {
            size,
            human_exists: vec![vec![false; size]; size],
            pet_exists: vec![vec![false; size]; size],
            block_exists: vec![vec![false; size]; size],
            distances: vec![vec![Self::INVALID_I32; size]; size],
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

    fn update(&mut self, humans: &[Human], pets: &[Pet]) {
        self.update_human_exists(humans);
        self.update_pet_exists(pets);
        self.distances = self.calc_distance(pets);
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
        let mut distance = vec![vec![Self::INVALID_I32; self.size]; self.size];
        let mut steps_from = vec![vec![Self::INVALID_VECTOR; self.size]; self.size];
        let mut queue: VecDeque<Vector> = VecDeque::new();
        queue.push_back(position.clone());
        distance[position.x as usize][position.y as usize] = 0;

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(x)) {
                if self.is_movable(&np)
                    && distance[np.x as usize][np.y as usize] == Self::INVALID_I32
                {
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
        let choice = |kind: &Kind| match kind {
            Kind::Cow | Kind::Pig | Kind::Rabbit => true,
            Kind::Dog => rand::thread_rng().gen_bool(1.0 / 3.0),
            Kind::Cat => rand::thread_rng().gen_bool(1.0 / 2.0),
            _ => false,
        };

        let mut distance = vec![vec![Self::INVALID_I32; self.size]; self.size];
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in pets.iter().filter(|x| choice(&x.kind)).map(|x| x.position) {
            distance[position.x as usize][position.y as usize] = 0;
            queue.push_back(position.clone());
        }

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(x)) {
                if self.is_movable(&np)
                    && distance[np.x as usize][np.y as usize] == Self::INVALID_I32
                {
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

    fn move_to(&mut self, position: &Vector) {
        if self.is_movable(&position) {
            self.human_exists[position.x as usize][position.y as usize] = true;
        }
    }

    fn block(&mut self, position: &Vector) {
        if self.is_blockable(&position) {
            self.block_exists[position.x as usize][position.y as usize] = true;
        }
    }
}

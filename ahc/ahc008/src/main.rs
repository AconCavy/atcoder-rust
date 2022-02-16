#![allow(dead_code)]

use std::collections::VecDeque;
use std::io;
use std::io::BufReader;

use itertools::Itertools;
use proconio::marker::Chars;
use proconio::source::line::LineSource;
use proconio::*;

const GRID_SIZE: usize = 30 + 2;
const GRID_MIN: i32 = 1;
const GRID_MAX: i32 = 30;
const TURN: usize = 300;

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
    for i in 0..m {
        input! {
            from &mut source,
            hx: i32,
            hy: i32,
        }

        let ax = GRID_MAX / m as i32 * i as i32 + 1;
        let ay = GRID_MAX / 2;
        humans.push(Human::new(hx, hy, ax, ay));
    }

    let mut grid = Grid::new(GRID_SIZE);
    grid.init();

    for _ in 0..TURN {
        grid.update(&humans, &pets);

        let mut answer = vec!['.'; m];
        for (i, human) in humans.iter_mut().enumerate() {
            answer[i] = human.act(&mut grid);
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Phase {
    Setup1,
    Wall,
    Setup2,
    Block,
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
    phase: Phase,
    axis: Vector,
}

impl Human {
    fn new(x: i32, y: i32, ax: i32, ay: i32) -> Self {
        Self {
            position: Vector::new(x, y),
            phase: Phase::Setup1,
            axis: Vector::new(ax, ay),
        }
    }

    fn act(&mut self, grid: &mut Grid) -> char {
        match self.phase {
            Phase::Setup1 => self.act_setup1(grid),
            Phase::Wall => self.act_wall(grid),
            Phase::Setup2 => self.act_setup2(grid),
            Phase::Block => self.act_block(grid),
        }
    }

    fn act_setup1(&mut self, grid: &mut Grid) -> char {
        let position = Vector::new(self.axis.x, grid.get_movable_min());
        match self.try_approach_to(grid, &position) {
            None => {
                self.phase = Phase::Wall;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_wall(&mut self, grid: &mut Grid) -> char {
        let d = 2;
        if self.axis.y - d <= self.position.y && self.position.y <= self.axis.y + d {
            return match self.try_move_to(grid, &Direction::Right) {
                None => '.',
                Some(result) => result,
            };
        }

        let up = self.position.neighbor(&Direction::Up);
        return if grid.block_exists[up.x as usize][up.y as usize] {
            let end = Vector::new(self.axis.x, grid.get_movable_max());
            if self.position == end {
                self.phase = Phase::Setup2;
                return '.';
            }

            match self.try_move_to(grid, &Direction::Right) {
                None => '.',
                Some(result) => result,
            }
        } else {
            match self.try_block(grid, &Direction::Up) {
                None => '.',
                Some(result) => result,
            }
        };
    }

    fn act_setup2(&mut self, grid: &mut Grid) -> char {
        let position = self.axis.clone();
        match self.try_approach_to(grid, &position) {
            None => {
                self.phase = Phase::Block;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_block(&mut self, grid: &mut Grid) -> char {
        let distance = grid.get_distance(&self.position);
        if distance == Grid::INVALID_I32 {
            return '.';
        }

        if distance > 5 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.position.neighbor(&direction);
                if grid.is_valid(&np) && grid.get_distance(&np) < distance {
                    if let Some(result) = self.try_move_to(grid, &direction) {
                        return result;
                    }
                }
            }
        } else if distance > 2 {
            let blocked = Direction::DIRECTION4
                .iter()
                .map(|x| self.position.neighbor(x))
                .filter(|v| grid.block_exists[v.x as usize][v.y as usize])
                .count();

            if blocked < 2 {
                for direction in Direction::DIRECTION4.iter() {
                    let np = self.position.neighbor(&direction);
                    if grid.is_valid(&np) && grid.get_distance(&np) < distance {
                        if let Some(result) = self.try_block(grid, &direction) {
                            return result;
                        }
                    }
                }
            }
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.position.neighbor(&direction);
            if grid.is_valid(&np) && grid.get_distance(&np) > distance {
                if let Some(result) = self.try_move_to(grid, &direction) {
                    return result;
                }
            }
        }
        '.'
    }

    fn try_approach_to(&mut self, grid: &mut Grid, position: &Vector) -> Option<char> {
        let distances = grid.calc_distance(&vec![position.clone()]);

        let get_distance = |grid: &Grid, position: &Vector| -> i32 {
            if grid.is_valid(&position) {
                distances[position.x as usize][position.y as usize]
            } else {
                Grid::INVALID_I32
            }
        };

        let distance = get_distance(grid, &self.position);
        if distance == Grid::INVALID_I32 {
            return None;
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.position.neighbor(&direction);
            if grid.is_valid(&np) && get_distance(grid, &np) < distance {
                if let Some(result) = self.try_move_to(grid, &direction) {
                    return Some(result);
                }
            }
        }

        None
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

        let targets = pets.iter().map(|x| x.position).collect_vec();
        self.distances = self.calc_distance(&targets);
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

    fn calc_distance(&self, positions: &[Vector]) -> Vec<Vec<i32>> {
        let mut distance = vec![vec![Self::INVALID_I32; self.size]; self.size];
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in positions {
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

    fn get_distance(&self, position: &Vector) -> i32 {
        if self.is_valid(&position) {
            self.distances[position.x as usize][position.y as usize]
        } else {
            Grid::INVALID_I32
        }
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

    fn get_movable_min(&self) -> i32 {
        1
    }

    fn get_movable_max(&self) -> i32 {
        self.size as i32 - 2
    }
}

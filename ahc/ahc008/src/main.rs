#![allow(dead_code)]

use itertools::Itertools;
use std::collections::VecDeque;
use std::io;
use std::io::BufReader;
use std::ops::{Index, IndexMut};

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
                1 => PetType::Cow,
                2 => PetType::Pig,
                3 => PetType::Rabbit,
                4 => PetType::Dog,
                5 => PetType::Cat,
                _ => PetType::None,
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

        let div = m.min(6) as i32;
        let ax = GRID_MAX / div * (i as i32 % div) + 1;
        let ay = GRID_MAX / 2;
        humans.push(Human::new(hx, hy, ax, ay));
    }

    let mut block_map = BlockMap::new(GRID_SIZE);
    block_map.init();

    let mut world = World::new(GRID_SIZE, humans, pets, block_map);
    world.init();

    for _ in 0..TURN {
        world.update();

        let answer: Vec<_> = world.act_humans();
        println!("{}", answer.into_iter().join(""));

        input! {
            from &mut source,
            actions: [Chars; n]
        }

        world.act_pets(&actions);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    fn neighbor(&self, direction: Direction) -> Vector {
        let v = direction.to_vector();
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PetType {
    None,
    Cow,
    Pig,
    Rabbit,
    Dog,
    Cat,
}

#[derive(Debug, Clone)]
struct Pet {
    position: Vector,
    pet_type: PetType,
}

impl Pet {
    fn new(x: i32, y: i32, pet_type: PetType) -> Self {
        Self {
            position: Vector { x, y },
            pet_type,
        }
    }

    fn act(&mut self, action: &[char]) {
        for c in action {
            self.position
                .translate(&Direction::from_char(*c).to_vector());
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum HumanState {
    Setup1,
    BlockOne,
    Setup2,
    BlockTwo,
}

#[derive(Debug, Clone)]
struct Human {
    position: Vector,
    state: HumanState,
    axis: Vector,
}

impl Human {
    fn new(x: i32, y: i32, ax: i32, ay: i32) -> Self {
        Self {
            position: Vector::new(x, y),
            state: HumanState::Setup1,
            axis: Vector::new(ax, ay),
        }
    }
}

struct World {
    size: usize,
    humans: Vec<Human>,
    pets: Vec<Pet>,
    block_map: BlockMap,
    human_exists: Grid<bool>,
    pet_exists: Grid<bool>,
    block_exists: Grid<bool>,
    distances_from_pets: Grid<i32>,
}

impl World {
    const INVALID_I32: i32 = -1;
    const INVALID_VECTOR: Vector = Vector {
        x: Self::INVALID_I32,
        y: Self::INVALID_I32,
    };

    fn new(size: usize, humans: Vec<Human>, pets: Vec<Pet>, block_map: BlockMap) -> Self {
        Self {
            size,
            humans,
            pets,
            block_map,
            human_exists: Grid::new(size, size, false),
            pet_exists: Grid::new(size, size, false),
            block_exists: Grid::new(size, size, false),
            distances_from_pets: Grid::new(size, size, 0),
        }
    }

    fn init(&mut self) {
        for i in 0..self.size {
            self.block_exists[(0, i)] = true;
            self.block_exists[(self.size - 1, i)] = true;
            self.block_exists[(i, 0)] = true;
            self.block_exists[(i, self.size - 1)] = true;
        }
    }

    fn update(&mut self) {
        self.human_exists.fill(false);
        for position in self.humans.iter().map(|x| x.position) {
            self.human_exists[&position] = true;
        }

        self.pet_exists.fill(false);
        for position in self.pets.iter().map(|x| x.position) {
            self.pet_exists[&position] = true;
        }

        let targets = self.pets.iter().map(|x| x.position).collect_vec();
        self.distances_from_pets = self.calc_distances(&targets);
    }

    fn act_humans(&mut self) -> Vec<char> {
        let mut result = vec!['.'; self.humans.len()];
        for i in 0..self.humans.len() {
            result[i] = match self.humans[i].state {
                HumanState::Setup1 => self.act_setup1(i),
                HumanState::BlockOne => self.act_block_one(i),
                HumanState::Setup2 => self.act_setup2(i),
                HumanState::BlockTwo => self.act_block_two(i),
            };
        }

        result
    }

    fn act_setup1(&mut self, idx: usize) -> char {
        let x = 5 * (idx as i32 % 6 + 1) + 1;
        let y = if self.humans[idx].position.y < self.size as i32 / 2 {
            0
        } else {
            30
        };
        let position = Vector::new(x, y);
        match self.approach_to(idx, &position) {
            None => {
                self.humans[idx].state = HumanState::BlockOne;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_setup2(&mut self, idx: usize) -> char {
        let x = self.humans[idx].position.x;
        let y = (8 * (idx % 3 + 1) - 1) as i32;
        let position = Vector::new(x, y);
        match self.approach_to(idx, &position) {
            None => {
                self.humans[idx].state = HumanState::BlockTwo;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_block(&mut self, idx: usize, distances: &Grid<i32>, distance: i32) -> char {
        if distance == 0 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.humans[idx].position.neighbor(*direction);
                if self.is_valid(&np) && self.get_distance(&distances, &np) > distance {
                    if let Some(result) = self.move_to(idx, *direction) {
                        return result;
                    }
                }
            }
        } else if distance == 1 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.humans[idx].position.neighbor(*direction);
                if self.is_valid(&np) && self.get_distance(&distances, &np) < distance {
                    if let Some(result) = self.block(idx, *direction) {
                        return result;
                    }
                }
            }
        } else {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.humans[idx].position.neighbor(*direction);
                if self.is_valid(&np) && self.get_distance(&distances, &np) < distance {
                    if let Some(result) = self.move_to(idx, *direction) {
                        return result;
                    }
                }
            }
        }

        '.'
    }

    fn act_block_one(&mut self, idx: usize) -> char {
        let target: Vec<_> = self
            .block_map
            .positions_one
            .iter()
            .filter(|x| !self.block_exists[*x])
            .map(|x| x.clone())
            .collect();

        let distances = self.calc_distances(&target);
        let distance = self.get_distance(&distances, &self.humans[idx].position);
        if distance == Self::INVALID_I32 || distance > 20 {
            self.humans[idx].state = HumanState::Setup2;
            return '.';
        }

        self.act_block(idx, &distances, distance)
    }

    fn act_block_two(&mut self, idx: usize) -> char {
        let exists_left = |pet_position: &Vector, block_position: &Vector| -> bool {
            block_position.y - 5 <= pet_position.y
                && pet_position.y <= block_position.y
                && block_position.x - 1 <= pet_position.x
                && pet_position.x <= block_position.x
        };

        let exists_right = |pet_position: &Vector, block_position: &Vector| -> bool {
            block_position.y <= pet_position.y
                && pet_position.y <= block_position.y + 5
                && block_position.x - 1 <= pet_position.x
                && pet_position.x <= block_position.x
        };

        let target: Vec<_> = self
            .block_map
            .positions_two
            .iter()
            .filter(|position| {
                !self.block_exists[*position]
                    && self.humans[idx].position.y - 2 <= position.y
                    && position.y <= self.humans[idx].position.y + 2
                    && self.pets.iter().any(|pet| {
                        if position.y % 8 == 6 {
                            exists_left(&pet.position, position)
                        } else {
                            exists_right(&pet.position, position)
                        }
                    })
            })
            .map(|x| x.clone())
            .collect();

        let distances = self.calc_distances(&target);
        let distance = self.get_distance(&distances, &self.humans[idx].position);
        if distance == Self::INVALID_I32 {
            return '.';
        }

        self.act_block(idx, &distances, distance)
    }

    fn act_pets(&mut self, actions: &[Vec<char>]) {
        for (pet, action) in self.pets.iter_mut().zip(actions.iter()) {
            pet.act(action);
        }
    }

    fn calc_distances(&self, positions: &[Vector]) -> Grid<i32> {
        let mut distance = Grid::new(self.size, self.size, World::INVALID_I32);
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in positions {
            distance[position] = 0;
            queue.push_back(position.clone());
        }

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(*x)) {
                if self.is_movable(&np) && distance[&np] == Self::INVALID_I32 {
                    distance[&np] = distance[&cp] + 1;
                    queue.push_back(np);
                }
            }
        }

        distance
    }

    fn get_distance(&self, grid: &Grid<i32>, position: &Vector) -> i32 {
        if self.is_valid(&position) {
            grid[position]
        } else {
            World::INVALID_I32
        }
    }

    fn is_valid(&self, position: &Vector) -> bool {
        0 <= position.x
            && position.x < self.size as i32
            && 0 <= position.y
            && position.y < self.size as i32
    }

    fn is_movable(&self, position: &Vector) -> bool {
        self.is_valid(position) && !self.block_exists[position]
    }

    fn is_blockable(&self, position: &Vector) -> bool {
        if !self.is_valid(position)
            || self.block_exists[position]
            || self.human_exists[position]
            || self.pet_exists[position]
        {
            return false;
        }

        for np in Direction::DIRECTION4.iter().map(|x| position.neighbor(*x)) {
            if self.is_valid(&np) && self.pet_exists[&np] {
                return false;
            }
        }

        true
    }

    fn move_to(&mut self, idx: usize, direction: Direction) -> Option<char> {
        let position = self.humans[idx].position.neighbor(direction);
        return if self.is_movable(&position) {
            self.human_exists[&position] = true;
            self.humans[idx].position = position;
            Some(direction.to_char())
        } else {
            None
        };
    }

    fn block(&mut self, idx: usize, direction: Direction) -> Option<char> {
        let position = self.humans[idx].position.neighbor(direction);
        return if self.is_blockable(&position) {
            self.block_exists[&position] = true;
            Some(direction.to_char().to_ascii_lowercase())
        } else {
            None
        };
    }

    fn approach_to(&mut self, idx: usize, position: &Vector) -> Option<char> {
        let distances = self.calc_distances(&vec![position.clone()]);
        let distance = self.get_distance(&distances, &self.humans[idx].position);
        if distance == World::INVALID_I32 {
            return None;
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.humans[idx].position.neighbor(*direction);
            if self.is_valid(&np) && self.get_distance(&distances, &np) < distance {
                if let Some(result) = self.move_to(idx, *direction) {
                    return Some(result);
                }
            }
        }

        None
    }

    fn get_movable_min(&self) -> i32 {
        1
    }

    fn get_movable_max(&self) -> i32 {
        self.size as i32 - 2
    }
}

struct Grid<T>
where
    T: Clone,
{
    x: usize,
    y: usize,
    values: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(x: usize, y: usize, e: T) -> Self {
        Self {
            x,
            y,
            values: vec![e; x * y],
        }
    }

    fn fill(&mut self, value: T) {
        for i in 0..self.values.len() {
            self.values[i] = value.clone();
        }
    }
}

impl<T: std::clone::Clone> Index<&Vector> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Vector) -> &Self::Output {
        &self.values[index.x as usize * self.y + index.y as usize]
    }
}

impl<T: std::clone::Clone> IndexMut<&Vector> for Grid<T> {
    fn index_mut(&mut self, index: &Vector) -> &mut Self::Output {
        &mut self.values[index.x as usize * self.y + index.y as usize]
    }
}

impl<T: std::clone::Clone> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.values[x * self.y + y]
    }
}

impl<T: std::clone::Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.values[(x * self.y + y) as usize]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockType {
    None,
    One,
    Two,
}

struct BlockMap {
    size: usize,
    map: Grid<BlockType>,
    positions_one: Vec<Vector>,
    positions_two: Vec<Vector>,
}

impl BlockMap {
    fn new(size: usize) -> Self {
        Self {
            size,
            map: Grid::new(size, size, BlockType::None),
            positions_one: Vec::new(),
            positions_two: Vec::new(),
        }
    }

    fn init(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                if i % 3 == 0 {
                    self.map[(i, j)] = match j % 8 {
                        6 | 0 => {
                            self.positions_one.push(Vector::new(i as i32, j as i32));
                            BlockType::One
                        }
                        _ => BlockType::None,
                    };
                } else if i % 3 == 1 {
                    self.map[(i, j)] = match j % 8 {
                        6 | 0 => {
                            self.positions_two.push(Vector::new(i as i32, j as i32));
                            BlockType::Two
                        }
                        _ => BlockType::None,
                    }
                } else {
                    self.map[(i, j)] = match j % 8 {
                        6 | 7 | 0 => BlockType::None,
                        _ => {
                            self.positions_one.push(Vector::new(i as i32, j as i32));
                            BlockType::One
                        }
                    };
                }
            }
        }

        for i in 0..self.size {
            self.map[(i, 0)] = BlockType::One;
            self.map[(i, self.size - 1)] = BlockType::One;
            self.map[(0, i)] = BlockType::One;
            self.map[(self.size - 1, i)] = BlockType::One;
        }
    }
}

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
    for _ in 0..m {
        input! {
            from &mut source,
            hx: i32,
            hy: i32,
        }

        humans.push(Human::new(hx, hy));
    }

    let mut blocks = Vec::with_capacity(GRID_SIZE * GRID_SIZE);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            blocks.push(Block::new_by_rule(i as i32, j as i32));
        }
    }

    let mut world = World::new(GRID_SIZE, humans, pets, blocks);
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
    SetupOne,
    BlockOne,
    SetupTwo,
    BlockTwo,
}

#[derive(Debug, Clone)]
struct Human {
    position: Vector,
    state: HumanState,
}

impl Human {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Vector::new(x, y),
            state: HumanState::SetupOne,
        }
    }
}

struct World {
    size: usize,
    humans: Vec<Human>,
    pets: Vec<Pet>,
    blocks: Vec<Block>,
    human_exists: Grid<bool>,
    pet_exists: Grid<bool>,
    block_exists: Grid<bool>,
    distances_from_pets: Grid<i32>,
}

impl World {
    const INVALID_DISTANCE: i32 = -1;

    fn new(size: usize, humans: Vec<Human>, pets: Vec<Pet>, blocks: Vec<Block>) -> Self {
        Self {
            size,
            humans,
            pets,
            blocks,
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
            self.human_exists[position] = true;
        }

        self.pet_exists.fill(false);
        for position in self.pets.iter().map(|x| x.position) {
            self.pet_exists[position] = true;
        }

        let targets = self.pets.iter().map(|x| x.position).collect_vec();
        self.distances_from_pets = self.calc_distances(&targets);
    }

    fn act_humans(&mut self) -> Vec<char> {
        let mut result = vec!['.'; self.humans.len()];
        for i in 0..self.humans.len() {
            result[i] = match self.humans[i].state {
                HumanState::SetupOne => self.act_setup_one(i),
                HumanState::BlockOne => self.act_block_one(i),
                HumanState::SetupTwo => self.act_setup_two(i),
                HumanState::BlockTwo => self.act_block_two(i),
            };
        }

        result
    }

    fn act_setup_one(&mut self, idx: usize) -> char {
        let x = 6 * (idx as i32 % 5) + 3;
        let y = if self.humans[idx].position.y < self.size as i32 / 2 {
            0
        } else {
            30
        };
        let position = Vector::new(x, y);
        match self.approach_to(idx, position) {
            None => {
                self.humans[idx].state = HumanState::BlockOne;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_setup_two(&mut self, idx: usize) -> char {
        let x = self.humans[idx].position.x;
        let y = 10 * (idx as i32 % 2 + 1);
        let position = Vector::new(x, y);
        match self.approach_to(idx, position) {
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
                if self.is_valid(np) && self.get_distance(&distances, np) > distance {
                    if let Some(result) = self.move_to(idx, *direction) {
                        return result;
                    }
                }
            }
        } else if distance == 1 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.humans[idx].position.neighbor(*direction);
                if self.is_valid(np) && self.get_distance(&distances, np) < distance {
                    if let Some(result) = self.block(idx, *direction) {
                        return result;
                    }
                }
            }
        } else {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.humans[idx].position.neighbor(*direction);
                if self.is_valid(np) && self.get_distance(&distances, np) < distance {
                    if let Some(result) = self.move_to(idx, *direction) {
                        return result;
                    }
                }
            }
        }

        '.'
    }

    fn act_block_one(&mut self, idx: usize) -> char {
        let filter_block_one = |block: &Block| match block.block_type {
            BlockType::One => true,
            _ => false,
        };

        let target: Vec<_> = self
            .blocks
            .iter()
            .filter(|x| filter_block_one(x))
            .map(|x| x.position)
            .filter(|x| !self.block_exists[*x])
            .collect();

        let distances = self.calc_distances(&target);
        let distance = self.get_distance(&distances, self.humans[idx].position);
        if distance == Self::INVALID_DISTANCE || distance > 20 {
            self.humans[idx].state = HumanState::SetupTwo;
            return '.';
        }

        self.act_block(idx, &distances, distance)
    }

    fn act_block_two(&mut self, idx: usize) -> char {
        let filter_block_two = |block: &Block| match block.block_type {
            BlockType::TwoL | BlockType::TwoR => true,
            _ => false,
        };

        let filter_by_y =
            |human: Vector, block: Vector| human.y - 2 <= block.y && block.y <= human.y + 2;

        let target: Vec<_> = self
            .blocks
            .iter()
            .filter(|block| {
                filter_block_two(block)
                    && !self.block_exists[block.position]
                    && filter_by_y(self.humans[idx].position, block.position)
                    && self
                        .pets
                        .iter()
                        .any(|x| block.is_targeted_by_rule(x.position))
                    && self
                        .humans
                        .iter()
                        .all(|x| !block.is_targeted_by_rule(x.position))
            })
            .map(|x| x.position)
            .filter(|x| !self.block_exists[*x])
            .collect();

        let distances = self.calc_distances(&target);
        let distance = self.get_distance(&distances, self.humans[idx].position);
        if distance == Self::INVALID_DISTANCE {
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
        let mut distance = Grid::new(self.size, self.size, World::INVALID_DISTANCE);
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in positions {
            distance[*position] = 0;
            queue.push_back(position.clone());
        }

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(*x)) {
                if self.is_movable(np) && distance[np] == Self::INVALID_DISTANCE {
                    distance[np] = distance[cp] + 1;
                    queue.push_back(np);
                }
            }
        }

        distance
    }

    fn get_distance(&self, grid: &Grid<i32>, position: Vector) -> i32 {
        if self.is_valid(position) {
            grid[position]
        } else {
            World::INVALID_DISTANCE
        }
    }

    fn is_valid(&self, position: Vector) -> bool {
        0 <= position.x
            && position.x < self.size as i32
            && 0 <= position.y
            && position.y < self.size as i32
    }

    fn is_movable(&self, position: Vector) -> bool {
        self.is_valid(position) && !self.block_exists[position]
    }

    fn is_blockable(&self, position: Vector) -> bool {
        if !self.is_valid(position)
            || self.block_exists[position]
            || self.human_exists[position]
            || self.pet_exists[position]
        {
            return false;
        }

        for np in Direction::DIRECTION4.iter().map(|x| position.neighbor(*x)) {
            if self.is_valid(np) && self.pet_exists[np] {
                return false;
            }
        }

        true
    }

    fn move_to(&mut self, idx: usize, direction: Direction) -> Option<char> {
        let position = self.humans[idx].position.neighbor(direction);
        return if self.is_movable(position) {
            self.human_exists[position] = true;
            self.humans[idx].position = position;
            Some(direction.to_char())
        } else {
            None
        };
    }

    fn block(&mut self, idx: usize, direction: Direction) -> Option<char> {
        let position = self.humans[idx].position.neighbor(direction);
        return if self.is_blockable(position) {
            self.block_exists[position] = true;
            Some(direction.to_char().to_ascii_lowercase())
        } else {
            None
        };
    }

    fn approach_to(&mut self, idx: usize, position: Vector) -> Option<char> {
        let distances = self.calc_distances(&vec![position]);
        let distance = self.get_distance(&distances, self.humans[idx].position);
        if distance == World::INVALID_DISTANCE {
            return None;
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.humans[idx].position.neighbor(*direction);
            if self.is_valid(np) && self.get_distance(&distances, np) < distance {
                if let Some(result) = self.move_to(idx, *direction) {
                    return Some(result);
                }
            }
        }

        None
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

impl<T: std::clone::Clone> Index<Vector> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vector) -> &Self::Output {
        &self.values[index.x as usize * self.y + index.y as usize]
    }
}

impl<T: std::clone::Clone> IndexMut<Vector> for Grid<T> {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
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
    TwoL,
    TwoR,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block {
    position: Vector,
    block_type: BlockType,
}

impl Block {
    fn new(x: i32, y: i32, block_type: BlockType) -> Self {
        Self {
            position: Vector { x, y },
            block_type,
        }
    }

    fn new_by_rule(x: i32, y: i32) -> Self {
        if x % 3 == 0 {
            match y % 11 {
                9 | 0 => Block::new(x, y, BlockType::One),
                _ => Block::new(x, y, BlockType::None),
            }
        } else if x % 3 == 1 {
            match y % 11 {
                9 => Block::new(x, y, BlockType::TwoR),
                0 => Block::new(x, y, BlockType::TwoL),
                _ => Block::new(x, y, BlockType::None),
            }
        } else {
            match y % 11 {
                9 | 10 | 0 => Block::new(x, y, BlockType::None),
                _ => Block::new(x, y, BlockType::One),
            }
        }
    }

    fn is_targeted_by_rule(&self, target: Vector) -> bool {
        match self.block_type {
            BlockType::None => false,
            BlockType::One => true,
            BlockType::TwoL => {
                self.position.y <= target.y
                    && target.y <= self.position.y + 9
                    && self.position.x - 1 <= target.x
                    && target.x <= self.position.x
            }
            BlockType::TwoR => {
                self.position.y - 9 <= target.y
                    && target.y <= self.position.y
                    && self.position.x - 1 <= target.x
                    && target.x <= self.position.x
            }
        }
    }
}

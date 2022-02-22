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

    let mut world = World::new(GRID_SIZE);
    world.init();

    for _ in 0..TURN {
        world.update(&humans, &pets);

        let answer: Vec<_> = humans.iter_mut().map(|x| x.act(&mut world)).collect();
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

    fn neighbor(&self, direction: &Direction) -> Vector {
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
    Wall,
    Setup2,
    Block,
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

    fn act(&mut self, world: &mut World) -> char {
        match self.state {
            HumanState::Setup1 => self.act_setup1(world),
            HumanState::Wall => self.act_wall(world),
            HumanState::Setup2 => self.act_setup2(world),
            HumanState::Block => self.act_block(world),
        }
    }

    fn act_setup1(&mut self, grid: &mut World) -> char {
        let position = Vector::new(self.axis.x, grid.get_movable_min());
        match self.try_approach_to(grid, &position) {
            None => {
                self.state = HumanState::Wall;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_wall(&mut self, world: &mut World) -> char {
        let d = 2;
        if self.axis.y - d <= self.position.y && self.position.y <= self.axis.y + d {
            return match self.try_move_to(world, &Direction::Right) {
                None => '.',
                Some(result) => result,
            };
        }

        let up = self.position.neighbor(&Direction::Up);
        return if world.block_exists[&up] {
            let end = Vector::new(self.axis.x, world.get_movable_max());
            if self.position == end {
                self.state = HumanState::Setup2;
                return '.';
            }

            match self.try_move_to(world, &Direction::Right) {
                None => '.',
                Some(result) => result,
            }
        } else {
            match self.try_block(world, &Direction::Up) {
                None => '.',
                Some(result) => result,
            }
        };
    }

    fn act_setup2(&mut self, world: &mut World) -> char {
        let position = self.axis.clone();
        match self.try_approach_to(world, &position) {
            None => {
                self.state = HumanState::Block;
                '.'
            }
            Some(result) => result,
        }
    }

    fn act_block(&mut self, world: &mut World) -> char {
        let distance = world.get_distance(&self.position);
        if distance == World::INVALID_I32 {
            return '.';
        }

        if distance > 5 {
            for direction in Direction::DIRECTION4.iter() {
                let np = self.position.neighbor(&direction);
                if world.is_valid(&np) && world.get_distance(&np) < distance {
                    if let Some(result) = self.try_move_to(world, &direction) {
                        return result;
                    }
                }
            }
        } else if distance > 2 {
            let blocked = Direction::DIRECTION4
                .iter()
                .map(|x| self.position.neighbor(x))
                .filter(|v| world.block_exists[v])
                .count();

            if blocked < 2 {
                for direction in Direction::DIRECTION4.iter() {
                    let np = self.position.neighbor(&direction);
                    if world.is_valid(&np) && world.get_distance(&np) < distance {
                        if let Some(result) = self.try_block(world, &direction) {
                            return result;
                        }
                    }
                }
            }
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.position.neighbor(&direction);
            if world.is_valid(&np) && world.get_distance(&np) > distance {
                if let Some(result) = self.try_move_to(world, &direction) {
                    return result;
                }
            }
        }
        '.'
    }

    fn try_approach_to(&mut self, world: &mut World, position: &Vector) -> Option<char> {
        let distances = world.calc_distance(&vec![position.clone()]);

        let get_distance = |grid: &World, position: &Vector| -> i32 {
            if grid.is_valid(&position) {
                distances[position]
            } else {
                World::INVALID_I32
            }
        };

        let distance = get_distance(world, &self.position);
        if distance == World::INVALID_I32 {
            return None;
        }

        for direction in Direction::DIRECTION4.iter() {
            let np = self.position.neighbor(&direction);
            if world.is_valid(&np) && get_distance(world, &np) < distance {
                if let Some(result) = self.try_move_to(world, &direction) {
                    return Some(result);
                }
            }
        }

        None
    }

    fn try_move_to(&mut self, grid: &mut World, direction: &Direction) -> Option<char> {
        let position = self.position.neighbor(&direction);
        return if grid.is_movable(&position) {
            grid.move_to(&position);
            self.position = position;
            Some(direction.to_char())
        } else {
            None
        };
    }

    fn try_block(&self, grid: &mut World, direction: &Direction) -> Option<char> {
        let position = self.position.neighbor(&direction);
        return if grid.is_blockable(&position) {
            grid.block(&position);
            Some(direction.to_char().to_ascii_lowercase())
        } else {
            None
        };
    }
}

struct World {
    size: usize,
    human_exists: Grid<bool>,
    pet_exists: Grid<bool>,
    block_exists: Grid<bool>,
    distances: Grid<i32>,
}

impl World {
    const INVALID_I32: i32 = -1;
    const INVALID_VECTOR: Vector = Vector {
        x: Self::INVALID_I32,
        y: Self::INVALID_I32,
    };

    fn new(size: usize) -> Self {
        Self {
            size,
            human_exists: Grid::new(size, size, false),
            pet_exists: Grid::new(size, size, false),
            block_exists: Grid::new(size, size, false),
            distances: Grid::new(size, size, 0),
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

    fn update(&mut self, humans: &[Human], pets: &[Pet]) {
        self.update_human_exists(humans);
        self.update_pet_exists(pets);

        let targets = pets.iter().map(|x| x.position).collect_vec();
        self.distances = self.calc_distance(&targets);
    }

    fn update_human_exists(&mut self, humans: &[Human]) {
        self.human_exists = Grid::new(self.size, self.size, false);
        for position in humans.iter().map(|x| x.position) {
            self.human_exists[&position] = true;
        }
    }

    fn update_pet_exists(&mut self, pets: &[Pet]) {
        self.pet_exists = Grid::new(self.size, self.size, false);
        for position in pets.iter().map(|x| x.position) {
            self.pet_exists[&position] = true;
        }
    }

    fn calc_distance(&self, positions: &[Vector]) -> Grid<i32> {
        let mut distance = Grid::new(self.size, self.size, World::INVALID_I32);
        let mut queue: VecDeque<Vector> = VecDeque::new();
        for position in positions {
            distance[position] = 0;
            queue.push_back(position.clone());
        }

        while let Some(cp) = queue.pop_front() {
            for np in Direction::DIRECTION4.iter().map(|x| cp.neighbor(x)) {
                if self.is_movable(&np) && distance[&np] == Self::INVALID_I32 {
                    distance[&np] = distance[&cp] + 1;
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

        for np in Direction::DIRECTION4.iter().map(|x| position.neighbor(x)) {
            if self.is_valid(&np) && self.pet_exists[&np] {
                return false;
            }
        }

        true
    }

    fn get_distance(&self, position: &Vector) -> i32 {
        if self.is_valid(&position) {
            self.distances[position]
        } else {
            World::INVALID_I32
        }
    }

    fn move_to(&mut self, position: &Vector) {
        if self.is_movable(&position) {
            self.human_exists[position] = true;
        }
    }

    fn block(&mut self, position: &Vector) {
        if self.is_blockable(&position) {
            self.block_exists[position] = true;
        }
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

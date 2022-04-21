#![allow(dead_code)]

use rand::Rng;
use std::time::{Duration, Instant};

pub trait AnnealingState {
    fn calc_score(&self) -> u64;
    fn modify(&self) -> Self;
    fn is_valid(&self) -> bool;
}

pub struct AnnealingSimulator {
    time_limit_millis: u64,
    start_temp: f64,
    end_temp: f64,
    random_seed: u64,
}

impl AnnealingSimulator {
    pub fn new(time_limit_millis: u64, start_temp: f64, end_temp: f64, random_seed: u64) -> Self {
        Self {
            time_limit_millis,
            start_temp,
            end_temp,
            random_seed,
        }
    }

    pub fn simulate<S: AnnealingState>(&self, init_state: S) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.time_limit_millis);
        let mut best_state = init_state;
        let mut best_score = best_state.calc_score();
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.random_seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.start_temp
                + (self.end_temp - self.start_temp) * elapsed / self.time_limit_millis as f64
        };

        let calc_prob = |best_score: u64, score: u64, temp: f64| -> f64 {
            ((best_score - score) as f64 / temp).exp()
        };

        while timer.elapsed() < end {
            let state = best_state.modify();
            if state.is_valid() {
                let score = state.calc_score();
                let temp = calc_temp(timer.elapsed().as_millis() as f64);
                if calc_prob(best_score, score, temp) >= rng.gen::<f64>() {
                    best_state = state;
                    best_score = score;
                }
            }
        }

        best_state
    }
}

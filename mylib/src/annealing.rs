#![allow(dead_code)]

use rand::Rng;
use std::time::{Duration, Instant};

pub trait AnnealingState {
    fn calc_score(&self) -> u64;
    fn modify(&self) -> Self;
}

pub struct AnnealingConfig {
    time_limit_millis: u64,
    start_temp: f64,
    end_temp: f64,
    random_seed: u64,
}

pub struct AnnealingSimulator {
    config: AnnealingConfig,
}

impl AnnealingSimulator {
    pub fn new(config: AnnealingConfig) -> Self {
        Self { config }
    }

    pub fn simulate<S: AnnealingState>(&self, init_state: S) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.config.time_limit_millis);
        let mut best_state = init_state;
        let mut best_score = best_state.calc_score();
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.config.random_seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.config.start_temp
                + (self.config.end_temp - self.config.start_temp) * elapsed
                    / self.config.time_limit_millis as f64
        };

        let calc_prob = |best_score: u64, score: u64, temp: f64| -> f64 {
            ((best_score - score) as f64 / temp).exp()
        };

        while timer.elapsed() < end {
            let state = best_state.modify();
            let score = state.calc_score();
            let temp = calc_temp(timer.elapsed().as_millis() as f64);

            if calc_prob(best_score, score, temp) >= rng.gen::<f64>() {
                best_state = state;
                best_score = score;
            }
        }

        best_state
    }
}

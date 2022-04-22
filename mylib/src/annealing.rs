#![allow(dead_code)]

use rand::Rng;
use std::time::{Duration, Instant};

pub trait AnnealingState {
    fn calc_score(&self) -> f64;
    fn modify(&self, rng: &mut rand::rngs::StdRng) -> Self;
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

    pub fn simulate<S: AnnealingState + Clone>(&self, init_state: S) -> S {
        let timer = Instant::now();
        let end = Duration::from_millis(self.time_limit_millis);
        let mut curr_state = init_state;
        let mut curr_score = curr_state.calc_score();
        let mut best_state = curr_state.clone();
        let mut best_score = curr_score;
        let mut rng_prob: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.random_seed);
        let mut rng_modify: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(self.random_seed);

        let calc_temp = |elapsed: f64| -> f64 {
            self.start_temp
                + (self.end_temp - self.start_temp) * elapsed / self.time_limit_millis as f64
        };

        let calc_prob = |score_from: f64, score_to: f64, temp: f64| -> f64 {
            let delta = (score_to - score_from).min(0.0) as f64;
            (delta / temp).exp()
        };

        while timer.elapsed() < end {
            let next_state = curr_state.modify(&mut rng_modify);
            if next_state.is_valid() {
                let next_score = next_state.calc_score();
                let temp = calc_temp(timer.elapsed().as_millis() as f64);
                if calc_prob(curr_score, next_score, temp) >= rng_prob.gen::<f64>() {
                    curr_state = next_state;
                    curr_score = next_score;

                    if curr_score >= best_score {
                        best_state = curr_state.clone();
                        best_score = curr_score;
                    }
                }
            }
        }

        best_state
    }
}

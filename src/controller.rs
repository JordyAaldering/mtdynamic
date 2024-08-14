mod direction;
mod selection;

use direction::Direction;
use selection::*;

use crate::letterbox::Sample;

pub struct Controller {
    n: i32,
    pub t1: Vec<f64>,
    t_last: f64,
    step_size: i32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads,
            t1: Vec::new(),
            t_last: 0.0,
            step_size: max_threads / 2,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            // TODO: maybe just have a function U -> V to get the best,
            // and a preceding function T -> U that selects a field from the samples
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {

        if self.t1.is_empty() {
            // Init
            self.n = self.max_threads;
            let tn = self.selection_algorithm.find_best(samples) as f64;
            self.t1.push(tn * self.n as f64);
            self.t_last = tn;
            self.step_direction = Direction::Down;
            self.step_size = self.max_threads / 2;

            self.n
        } else {
            // Update
            let tn = self.selection_algorithm.find_best(samples) as f64;
            let t1 = *self.t1.last().unwrap();

            if t1 / tn < (1.0 - self.corridor_width) * self.n as f64 {
                self.step_direction = Direction::Down;
                self.step_size = i32::max(1, self.n / 2);
            } else {
                let t1_new = tn * self.n as f64;
                if t1_new < t1 {
                    println!("Approximation of t1 updated from {} to {}", t1, t1_new);
                    self.t1.push(t1_new);
                }

                if tn > self.t_last {
                    self.step_direction = -self.step_direction;
                }

                self.step_size = i32::max(1, self.step_size / 2);
            }

            self.n += self.step_direction * self.step_size;
            self.n = i32::max(1, i32::min(self.max_threads, self.n));

            self.t_last = tn;
            self.n
        }

    }
}

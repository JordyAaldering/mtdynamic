mod direction;
mod selection;

use direction::Direction;
use selection::{FrequencyDist, SelectionAlgorithm};

use crate::util::Clamped;

pub struct Controller {
    n: Clamped<i32>,
    t1: Option<u64>,
    t_last: u64,
    step_size: Clamped<i32>,
    step_direction: Direction,
    // Settings
    corridor_width: f64,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: Clamped::new(max_threads, 1, max_threads),
            t1: None,
            t_last: 0,
            step_size: Clamped::new(max_threads / 2, 1, max_threads / 2),
            step_direction: Direction::Down,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5))
        }
    }

    pub fn adjust_threads(&mut self, energy_consumed: Vec<u64>) -> i32 {
        let tn = self.selection_algorithm.find_best_time(energy_consumed);

        if let Some(t1) = self.t1 {
            // Update
            self.n += self.step_direction as i32 * self.step_size.get();

            let improvement = t1 as f64 / tn as f64;
            if improvement < self.n.get() as f64 * (1.0 - self.corridor_width) {
                self.step_direction = Direction::Down;
                self.step_size = self.n / 2;
            } else {
                if improvement > self.n.get() as f64 {
                    self.t1 = Some(tn * self.n.get() as u64);
                }

                if tn > self.t_last {
                    self.step_direction = -self.step_direction;
                }

                self.step_size /= 2;
            }
        } else {
            // Init
            self.t1 = Some(tn * self.n.get() as u64);
        }

        self.t_last = tn;
        self.n.get()
    }
}

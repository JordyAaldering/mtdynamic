mod direction;
mod selection;

use crate::letterbox::Sample;
use direction::Direction;
use selection::*;

const NUM_BUCKETS: usize = 5;

pub struct Controller {
    n: f32,
    //t_best: u64,
    //t_best_thread_count: u64,
    t_best_buckets: Vec<(u64, u32)>, // First value: energy, second value: thread count
    t_last: u64,
    // TODO: step size (and n) as a float, so that we have less variation over time
    // e.g., now we can have [7,8,9,8,7,8,9,8,7,8,9] for a very long time
    // it would be nice if the longer it stays like that, the less it changes
    // of course we still want it to change sometimes to check for improvements
    // Potentially, after the value becomes very small make a large jump,
    // in order to escape local minima
    step_size: f32,
    step_direction: Direction,
    // Settings
    max_threads: i32,
    corridor_width: f32,
    selection_algorithm: Box<dyn SelectionAlgorithm>,
}

fn energy_score(sample: &Sample) -> u64 {
    return sample.energy_uj;

    #[allow(unreachable_code)]
    if sample.usertime_ns >= sample.realtime_ns {
        sample.energy_uj
    } else {
        let frac = sample.usertime_ns as f32 / sample.realtime_ns as f32;
        (sample.energy_uj as f32 * frac) as u64
    }
}

fn user_frac(sample: &Sample) -> f32 {
    sample.usertime_ns as f32 / sample.realtime_ns as f32
}

impl Controller {
    pub fn new(max_threads: i32) -> Controller {
        Controller {
            n: max_threads as f32,
            //t_best: u64::MAX,
            //t_best_thread_count: max_threads as u64,
            t_best_buckets: vec![(u64::MAX, max_threads as u32); NUM_BUCKETS],
            t_last: u64::MAX,
            step_size: max_threads as f32,
            step_direction: Direction::Down,
            // Settings
            max_threads,
            corridor_width: 0.5,
            selection_algorithm: Box::new(FrequencyDist::new(5)),
        }
    }

    pub fn adjust_threads(&mut self, samples: Vec<Sample>) -> i32 {
        let bucket = self.get_bucket(&samples);
        let (t_best, t_best_thread_count) = self.t_best_buckets[bucket];

        let samples = samples.iter().map(energy_score).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = t_best as f32 / tn as f32;

        if speedup < 1.0 - self.corridor_width {
            // Fallen outside the corridor
            // Move up or down depending on where the best thread count was
            if tn > self.t_last {
                // The previous iteration performed much better; reverse direction
                self.step_direction = -self.step_direction;

                self.step_size *= 2.0;
            } else {
                // Otherwise we move towards our estimated optimum
                self.step_direction = Direction::towards(self.n, t_best_thread_count as f32);

                let diff = f32::abs(self.n - t_best_thread_count as f32);
                self.step_size = 0.75 * diff as f32;
            }

            println!("Fallen outside the corridor (speedup = {}), step size to {}", speedup, self.step_size);
        } else {
            if speedup > 1.0 / (1.0 - self.corridor_width) {
                println!("Went above the corridor (speedup = {}), step size to {}", speedup, self.n / 2.0);
                self.step_size = self.n as f32; // Will be n / 2 at the end of this block
            }

            if tn < t_best {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("T_best[{}] updated to {} at {} threads", bucket, tn, self.n.round() as i32);
                self.t_best_buckets[bucket] = (tn, self.n.round() as u32);
            }

            if tn > self.t_last {
                // The previous iteration performed better; reverse direction
                self.step_direction = -self.step_direction;
            }

            self.step_size /= 2.0;
        }

        self.t_last = tn;

        self.n += self.step_direction * self.step_size;
        self.n = f32::min(self.n, self.max_threads as f32);
        self.n.round() as i32
    }

    // The original runtime-based implementation, we use this for comparison
    /*#[allow(dead_code)]
    pub fn adjust_threads_runtime(&mut self, samples: Vec<Sample>) -> i32 {
        let samples = samples.into_iter().map(|sample| sample.realtime_ns).collect();
        let tn = self.selection_algorithm.find_best(samples);

        let speedup = self.t_best as f32 / tn as f32;
        if speedup < (1.0 - self.corridor_width) * self.n as f32 {
            // We have fallen outside the corridor
            self.step_direction = Direction::Down;
            self.step_size = i32::max(1, self.n / 2);
        } else {
            if speedup > self.n as f32 {
                // In the initial iteration t1 and t_last as u64::MAX so we
                // reach this condition, an initialize t1 with a real value
                println!("Approximation of t1 updated to {}", tn * self.n as u64);
                self.t_best = tn * self.n as u64;
            }

            if tn > self.t_last {
                self.step_direction = -self.step_direction;
            }

            self.step_size = i32::max(1, self.step_size / 2);
        }

        self.n = self.next_n();
        self.t_last = tn;
        self.n
    }*/

    fn get_bucket(&self, samples: &Vec<Sample>) -> usize {
        let user_frac: f32 = samples.iter().map(user_frac).sum();
        let user_frac = user_frac / samples.len() as f32;
        usize::min((user_frac * NUM_BUCKETS as f32) as usize, NUM_BUCKETS - 1)
    }
}

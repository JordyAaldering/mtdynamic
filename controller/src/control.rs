pub mod corridor_controller;
pub mod delta_controller;
pub mod genetic_controller;

use crate::message::{Demand, Sample};

pub trait Controller {
    fn sample_received(&mut self, sample: Sample);

    fn next_demand(&mut self) -> Demand;
}

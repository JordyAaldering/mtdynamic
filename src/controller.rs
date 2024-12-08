mod controller_energy;
mod controller_runtime;

pub use controller_energy::EnergyController;
pub use controller_runtime::RuntimeController;

pub trait Controller {
    fn adjust_threads(&mut self, samples: Vec<f32>) -> i32;
}

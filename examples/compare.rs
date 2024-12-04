#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let pin_threads: bool = args[2].parse().unwrap();
    let max_threads: usize = args[3].parse().unwrap();
    let mode: char = args[4].chars().next().unwrap();

    let mut mtd = match mode {
        's' => Mtd::fixed_controller(max_threads),
        'e' => Mtd::energy_controller(max_threads, 10),
        'r' => Mtd::runtime_controller(max_threads),
        s => unreachable!("{}", s),
    };

    let mut rapl = Rapl::now(false).unwrap();

    let x = black_box(Matrix::random(size, size));
    let y = black_box(Matrix::random(size, size));

    let mut runtimes = Vec::with_capacity(200);
    let mut energies = Vec::with_capacity(200);

    for _ in 0..200 {
        let num_threads = mtd.num_threads() as usize;
        let pool = threadpool(num_threads, pin_threads);

        rapl.reset();
        let instant = Instant::now();

        let _ = black_box(mtd.install(|| pool.install(|| x.mul(&y))));

        let runtime = instant.elapsed();
        let energy = rapl.elapsed();

        let runtime = runtime.as_secs_f32();
        let energy = energy.values().sum::<f32>();
        runtimes.push(runtime);
        energies.push(energy);
    }

    let runtime = statistical::mean(&runtimes);
    let energy = statistical::mean(&energies);
    let runtime_sd = statistical::population_standard_deviation(&runtimes, Some(runtime));
    let energy_sd = statistical::population_standard_deviation(&energies, Some(energy));

    let threads_str = match mode {
        'e' => "mt".to_string(),
        'r' => "rt".to_string(),
        _ => max_threads.to_string(),
    };
    println!("{},{},{},{},{},{},{}", threads_str, size, pin_threads, runtime, runtime_sd, energy, energy_sd);
}

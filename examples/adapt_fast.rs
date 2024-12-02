#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let max_threads: usize = args[1].parse().unwrap();
    let mode = args[2].chars().next().unwrap();

    const CYCLES: [(usize, bool); 6] = [
        ( 500, false),
        (1000, false),
        (1500, false),
        ( 500, true),
        (1000, true),
        (1500, true),
    ];

    let mut mtd = match mode {
        's' => Mtd::fixed_controller(max_threads),
        'e' => Mtd::energy_controller(max_threads, 10),
        'r' => Mtd::runtime_controller(max_threads),
        s => unreachable!("{}", s),
    };

    let mut rapl = Rapl::now(false).unwrap();

    for (size, pin_threads) in CYCLES {
        let x = black_box(Matrix::random(size, size));
        let y = black_box(Matrix::random(size, size));

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
            println!("{},{},{},{},{},{}", mode, size, pin_threads, mtd.num_threads, runtime, energy)
        }
    }
}

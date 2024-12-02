#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use mtdynamic::Mtd;
use rapl_energy::{EnergyProbe, Rapl};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let (max_threads, do_dynamic) = if let Some(max_threads) = args.get(1) {
        (max_threads.parse().unwrap(), false)
    } else {
        (16, true)
    };

    const CYCLES: [(usize, bool); 6] = [
        (1150, false),
        (1100, false),
        (1050, false),
        (1050, true),
        (1100, true),
        (1150, true),
    ];

    let mut mtd = if do_dynamic {
        Mtd::energy_controller(max_threads, 10)
    } else {
        Mtd::fixed_controller(max_threads)
    };

    let mut rapl = Rapl::now(false).unwrap();

    println!("size,pin,threads,runtime,energy");

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
            println!("{},{},{},{},{}", size, pin_threads, mtd.num_threads, runtime, energy);
        }
    }
}

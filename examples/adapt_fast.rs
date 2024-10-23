#[path = "util/util.rs"]
mod util;
use util::*;

use std::{hint::black_box, time::Instant};

use cpu_time::ProcessTime;
use mtdynamic::Mtd;
use rapl_energy::Rapl;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let print_intermediate: bool = args[1].parse().unwrap();
    let mut max_threads = 16;
    let mut dynamic = true;
    if args.len() > 2 {
        max_threads = args[2].parse().unwrap();
        dynamic = false;
    }

    const CYCLES: [(usize, bool, usize); 16] = [
        // Without pinning
        ( 500, false, 1000),
        ( 750, false,  750),
        (1000, false,  500),
        (1250, false,  150),
        (1500, false,  100),
        ( 500, false, 1000),
        ( 750, false,  750),
        (1000, false,  500),
        // With pinning
        ( 500, true, 1000),
        ( 750, true,  750),
        (1000, true,  500),
        (1250, true,  150),
        (1500, true,  100),
        ( 500, true, 1000),
        ( 750, true,  750),
        (1000, true,  500),
    ];

    let mut mtd = Mtd::energy_controller(max_threads, 10);
    let mut rapl = Rapl::now().unwrap();

    if print_intermediate {
        println!("size,pin,threads,runtime,usertime,energy");
    }

    let mut real_total = 0.0;
    let mut user_total = 0.0;
    let mut rapl_total = 0.0;

    for (size, pin_threads, iter) in CYCLES {
        for _ in 0..iter {
            let x = black_box(Matrix::random(size, size));
            let y = black_box(Matrix::random(size, size));

            rapl.reset();
            let user = ProcessTime::now();
            let real = Instant::now();

            let pool = threadpool(mtd.num_threads() as usize, pin_threads);
            let _ = if dynamic {
                black_box(mtd.install(|| pool.install(|| black_box(x.mul(&y)))))
            } else {
                pool.install(|| black_box(x.mul(&y)))
            };

            let real = real.elapsed();
            let user = user.elapsed();
            let rapl = rapl.elapsed();

            let real = real.as_secs_f32();
            let user = user.as_secs_f32();
            let energy: f32 = rapl.values().sum();
            real_total += real;
            user_total += user;
            rapl_total += energy;

            if print_intermediate {
                println!("{},{},{},{:},{:},{:}", size, pin_threads, mtd.num_threads, real, user, energy);
            }
        }
    }

    if !print_intermediate {
        println!("{},{},{}", real_total, user_total, rapl_total);
    }
}

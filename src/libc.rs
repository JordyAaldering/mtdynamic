use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::fs;
use std::io::Write;

use crate::sample::{Sample, SampleInstant};
use crate::Mtd;

#[repr(C)]
struct MTDs {
    max_threads: usize,
    runtime_based: i32,
    samples_per_update: usize,
    mtds: HashMap<String, (Mtd, Vec<(Sample, f32)>)>,
}

#[no_mangle]
extern "C" fn MTDcreate(max_threads: usize, runtime_based: i32, samples_per_update: usize) -> *mut MTDs {
    let mtds = MTDs { max_threads, runtime_based, samples_per_update, mtds: HashMap::new() };
    Box::into_raw(Box::new(mtds))
}

#[no_mangle]
extern "C" fn MTDstart(mtd: *mut &mut MTDs, funname: *const c_char) -> Box<SampleInstant> {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if !mtd.mtds.contains_key(&funname) {
        let controller = if mtd.runtime_based == 1 {
            Mtd::runtime_controller(mtd.max_threads)
        } else {
            Mtd::energy_controller(mtd.max_threads, mtd.samples_per_update)
        };
        mtd.mtds.insert(funname.clone(), (controller, Vec::new()));
    }

    Box::new(SampleInstant::now())
}

#[no_mangle]
extern "C" fn MTDstop(mtd: *mut &mut MTDs, now: Box<SampleInstant>, funname: *const c_char) {
    let sample = now.elapsed();

    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    let (controller, history) = mtd.mtds.get_mut(&funname).unwrap();

    history.push((sample.clone(), controller.num_threads));
    controller.update(sample);
}

#[no_mangle]
extern "C" fn MTDnumThreads(mtd: *mut &mut MTDs, funname: *const c_char) -> i32 {
    let mtd = unsafe { std::ptr::read(mtd) };
    let funname = unsafe { CStr::from_ptr(funname) };
    let funname = funname.to_str().unwrap().to_string();

    if let Some((controller, _)) = mtd.mtds.get_mut(&funname) {
        controller.num_threads()
    } else {
        mtd.max_threads as i32
    }
}

#[no_mangle]
extern "C" fn MTDfree(mtd: *mut MTDs) {
    let mtd = unsafe { std::ptr::read(mtd) };

    let (name, (_, history)) = mtd.mtds
        .into_iter()
        .max_by_key(|(_, (_, history))| history.iter().map(|(sample, _)| sample.runtime).sum::<f32>().ceil() as i32)
        .unwrap();

    if mtd.samples_per_update < 999 {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}_{}.csv", if mtd.runtime_based == 1 { "rt" } else { "mt" }, name.chars().take(100).collect::<String>()))
            .unwrap();

        for (sample, tc) in &history {
            let _ = writeln!(file, "{},{},{}", tc, sample.runtime, sample.energy);
        }
    }

    let runtimes = history.iter().map(|(sample, _)| sample.runtime).collect::<Vec<_>>();
    let energies = history.iter().map(|(sample, _)| sample.energy).collect::<Vec<_>>();

    let runtime_avg = statistical::mean(&runtimes);
    let energy_avg = statistical::mean(&energies);
    let runtime_sd = statistical::population_standard_deviation(&runtimes, None);
    let energy_sd = statistical::population_standard_deviation(&energies, None);

    println!("{:.8},{:.8},{:.8},{:.8}", runtime_avg, runtime_sd, energy_avg, energy_sd);
}

use std::cmp::Ordering;

use crate::letterbox::Sample;

pub trait SelectionAlgorithm {
    fn find_best(&self, samples: Vec<Sample>) -> u64;
}

pub struct Median {}

impl SelectionAlgorithm for Median {
    fn find_best(&self, samples: Vec<Sample>) -> u64 {
        let idx = samples.len() / 2;
        let mut samples = samples.into_iter()
            .map(|sample| sample.energy_uj)
            .collect::<Vec<u64>>();
        samples.sort();
        samples[idx]
    }
}

pub struct Average {}

impl SelectionAlgorithm for Average {
    fn find_best(&self, samples: Vec<Sample>) -> u64 {
        let len = samples.len() as u64;
        samples.into_iter()
            .map(|sample| sample.energy_uj)
            .sum::<u64>() / len
    }
}

pub struct Pareto {}

impl SelectionAlgorithm for Pareto {
    fn find_best(&self, samples: Vec<Sample>) -> u64 {
        //let realtime_max = samples.iter().map(|x| x.realtime_ns).max().unwrap() as f64;
        //let energy_max = samples.iter().map(|x| x.energy_uj).max().unwrap() as f64;

        let l2_min = samples.into_iter()
            .map(|sample|
                f64::sqrt(f64::powi(f64::abs(sample.usertime_ns as f64 / sample.realtime_ns as f64 - sample.energy_uj as f64), 2) as f64)
            )
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap();
        l2_min as u64
    }
}

pub struct FrequencyDist {
    num_ranges: usize,
}

impl FrequencyDist {
    #[allow(dead_code)]
    pub fn new(num_ranges: usize) -> Self {
        FrequencyDist { num_ranges }
    }

    fn get_distribution_maximums(&self, samples: &Vec<u64>) -> Vec<u64> {
        let min = samples[0];
        let max = samples[samples.len() - 1];
        let dist_size = (max - min) / self.num_ranges as u64;
        let mut res = (1..=self.num_ranges as u64)
            .map(|i| min + dist_size * i)
            .collect::<Vec<u64>>();
        res[self.num_ranges - 1] = max;
        res
    }
}

impl SelectionAlgorithm for FrequencyDist {
    fn find_best(&self, samples: Vec<Sample>) -> u64 {
        let mut samples = samples.into_iter()
            .map(|sample| sample.energy_uj)
            .collect::<Vec<u64>>();
        samples.sort();

        let dist_max = self.get_distribution_maximums(&samples);
        let mut dist = vec![Vec::new(); self.num_ranges];
        let mut dist_index = 0;
        for x in samples {
            while x > dist_max[dist_index] {
                dist_index += 1;
            }

            dist[dist_index].push(x);
        }

        //println!("{:?}", dist);
        let biggest_dist = dist.into_iter().max_by_key(Vec::len).unwrap();
        biggest_dist[0]
    }
}

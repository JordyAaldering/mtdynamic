use std::sync::{Arc, Mutex};

use super::Controller;

pub struct GeneticController {
    max_threads: i32,
    population: Vec<Chromosome>,
    sample_index: usize,
    config: Arc<Mutex<dyn GeneticControllerConfig>>,
}

pub trait GeneticControllerConfig {
    fn population_size(&self) -> usize;
    fn survival_rate(&self) -> f32;
    fn mutation_rate(&self) -> f32;
    fn immigration_rate(&self) -> f32;

    fn survival_count(&self) -> usize {
        (self.population_size() as f32 * self.survival_rate()).round() as usize
    }

    fn immigration_count(&self) -> usize {
        (self.population_size() as f32 * self.immigration_rate()).round() as usize
    }
}

impl GeneticController {
    pub fn new(max_threads: i32, config: Arc<Mutex<dyn GeneticControllerConfig>>) -> Self {
        let population_size = config.lock().unwrap().population_size();
        // Instead of randomly initialized values, use an even spread over valid thread-counts to
        // reduce duplication and increase the chances of finding an optimum immediately.
        let population = (0..population_size).map(|i| {
                let num_threads = 1 + (i as f64 * (max_threads - 1) as f64 / (population_size - 1) as f64).round() as i32;
                Chromosome::new(num_threads)
            }).collect();

        Self {
            max_threads,
            population,
            sample_index: 0,
            config,
        }
    }

    fn sort(&mut self, scores: Vec<f32>) {
        let mut permutation = permutation::sort_by(&scores, |a, b| a.partial_cmp(b).unwrap());
        permutation.apply_slice_in_place(&mut self.population);
    }
}

impl Controller for GeneticController {
    fn evolve(&mut self, scores: Vec<f32>) {
        self.sort(scores);

        let population_size = self.config.lock().unwrap().population_size();
        let survival_count = self.config.lock().unwrap().survival_count();
        let immigration_count = self.config.lock().unwrap().immigration_count();
        let immigration_start = population_size - immigration_count;

        // Replace chromosomes by children of the best performing chromosomes
        for i in survival_count..immigration_start {
            let parent1 = &self.population[rand::random_range(0..survival_count)];
            let parent2 = &self.population[rand::random_range(0..survival_count)];
            let mut child = parent1.crossover(&parent2);
            if rand::random_range(0.0..1.0) < self.config.lock().unwrap().mutation_rate() {
                child.mutate(self.max_threads);
            }

            self.population[i] = child;
        }

        // Fill remaining chromosomes by immigration
        for i in immigration_start..population_size {
            self.population[i] = Chromosome::rand(self.max_threads);
        }

        // We want to sort the population by recommended thread-count
        // here, to minimise changes in the running program.
        self.population.sort();
    }

    fn num_threads(&mut self) -> i32 {
        // Use the number of samples to determine the current index into the population.
        // The population is reset every `population_size` iterations.
        // In between, we want every chromosome to be applied once.
        let num_threads = self.population[self.sample_index].num_threads;
        self.sample_index += 1;
        num_threads
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chromosome {
    pub num_threads: i32,
}

impl Chromosome {
    fn new(num_threads: i32) -> Self {
        Self { num_threads }
    }

    /// Generate a random chromosome for immigration
    fn rand(max_threads: i32) -> Self {
        let num_threads = rand::random_range(1..=max_threads);
        Self::new(num_threads)
    }

    fn crossover(&self, other: &Self) -> Self {
        Self {
            num_threads: (self.num_threads + other.num_threads) / 2,
        }
    }

    /// Add or subtract one thread
    fn mutate(&mut self, max_threads: i32) {
        self.num_threads += rand::random_range(0..=1) * 2 - 1;
        self.num_threads = self.num_threads.max(1).min(max_threads)
    }
}

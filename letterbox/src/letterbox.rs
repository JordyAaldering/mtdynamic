use std::{collections::HashMap, mem};

pub use controller::*;

use crate::message::*;

pub struct Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    build: F,
    letterbox: HashMap<i32, (Vec<f32>, Box<dyn Controller>)>,
}

impl<F> Letterbox<F>
    where F: Fn(Request) -> Box<dyn Controller>
{
    pub fn new(build: F) -> Self {
        Self { build, letterbox: HashMap::new() }
    }

    pub fn try_get_demand(&mut self, req: Request) -> i32 {
        self.letterbox.entry(req.region_uid)
            .or_insert_with(|| (Vec::new(), (self.build)(req)))
            .1.num_threads()
    }

    pub fn get_demand(&mut self, region_uid: i32) -> i32 {
        self.letterbox.get_mut(&region_uid).unwrap()
            .1.num_threads()
    }

    pub fn update(&mut self, region_uid: i32, score: f32) {
        let (scores, controller) = self.letterbox
            .get_mut(&region_uid)
            .expect("Letterbox not initialized");

        scores.push(score);
        if scores.len() >= 20 /* TODO: population size */ {
            let mut scores_swap = Vec::with_capacity(20);
            mem::swap(scores, &mut scores_swap);
            controller.evolve(scores_swap);
        }
    }
}

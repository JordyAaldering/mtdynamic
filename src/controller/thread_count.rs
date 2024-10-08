use super::Direction;

pub struct ThreadCount {
    n: f64,
    max: f64,
}

impl ThreadCount {
    pub fn new(max: i32) -> Self {
        Self { n: max as f64, max: max as f64 }
    }

    pub fn adjust(&mut self, step_direction: Direction, step_size: f64) -> bool {
        let prev = self.n;
        self.n += step_direction * step_size;
        self.clamp();

        let changed = prev.round() != self.n.round();
        changed
    }

    fn clamp(&mut self) {
        self.n = self.n.max(1.0).min(self.max);
    }
}

impl std::ops::Deref for ThreadCount {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.n
    }
}

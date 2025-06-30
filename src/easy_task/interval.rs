use crate::tools::rtweekend::INFINITY;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn new_interval(a: Interval, b: Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[allow(dead_code)]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    #[allow(dead_code)]
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Add<f64> for Interval {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        Interval {
            min: self + rhs.min,
            max: self + rhs.max,
        }
    }
}
pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
#[allow(dead_code)]
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

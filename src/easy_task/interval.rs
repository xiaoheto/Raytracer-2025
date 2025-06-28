use crate::tools::rtweekend;

#[derive(Default, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    #[allow(dead_code)]
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
}
#[allow(dead_code)]
pub const EMPTY: Interval = Interval {
    min: rtweekend::INFINITY,
    max: -rtweekend::INFINITY,
};
#[allow(dead_code)]
pub const UNIVERSE: Interval = Interval {
    min: -rtweekend::INFINITY,
    max: rtweekend::INFINITY,
};

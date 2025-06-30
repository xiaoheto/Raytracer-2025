use super::interval;
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3};
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn new(mut x: Interval, mut y: Interval, mut z: Interval) -> Self {
        let delta = 0.0001;
        if x.size() < delta {
            x = x.expand(delta);
        }
        if y.size() < delta {
            y = y.expand(delta);
        }
        if z.size() < delta {
            z = z.expand(delta);
        }
        Self { x, y, z }
    }

    pub fn new_point(a: Point3, b: Point3) -> Self {
        let mut x = if a[0] < b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let mut y = if a[1] < b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let mut z = if a[2] < b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        let delta = 0.0001;
        if x.size() < delta {
            x = x.expand(delta);
        }
        if y.size() < delta {
            y = y.expand(delta);
        }
        if z.size() < delta {
            z = z.expand(delta);
        }
        Self { x, y, z }
    }

    pub fn new_aabb(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: Interval::new_interval(box0.x, box1.x),
            y: Interval::new_interval(box0.y, box1.y),
            z: Interval::new_interval(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    pub fn hit(&self, r: Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis as i32);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() && self.x.size() > self.z.size() {
            0
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}

impl Add<Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        Aabb {
            x: self.x() + rhs.x,
            y: self.y() + rhs.y,
            z: self.z() + rhs.z,
        }
    }
}

pub const EMPTY: Aabb = Aabb {
    x: interval::EMPTY,
    y: interval::EMPTY,
    z: interval::EMPTY,
};
#[allow(dead_code)]
pub const UNIVERSE: Aabb = Aabb {
    x: interval::UNIVERSE,
    y: interval::UNIVERSE,
    z: interval::UNIVERSE,
};

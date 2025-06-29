use super::interval;
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::Point3;

#[derive(Default, Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new_point(a: Point3, b: Point3) -> Self {
        Self {
            x: if a[0] < b[0] {
                Interval::new(a[0], b[0])
            } else {
                Interval::new(b[0], a[0])
            },
            y: if a[1] < b[1] {
                Interval::new(a[1], b[1])
            } else {
                Interval::new(b[1], a[1])
            },
            z: if a[2] < b[2] {
                Interval::new(a[2], b[2])
            } else {
                Interval::new(b[2], a[2])
            },
        }
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
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
}

pub const EMPTY: Aabb = Aabb {
    x: interval::EMPTY,
    y: interval::EMPTY,
    z: interval::EMPTY,
};
pub const UNIVERSE: Aabb = Aabb {
    x: interval::UNIVERSE,
    y: interval::UNIVERSE,
    z: interval::UNIVERSE,
};

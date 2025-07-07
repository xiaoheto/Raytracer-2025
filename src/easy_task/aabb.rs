use crate::easy_task;
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn pad(&self) -> Self {
        // 返回一个没有边小于某个 delta 的 AABB，如果需要则填充。
        let delta = 0.0001;
        let new_x = if self.x.size() < delta {
            self.x.expand(delta)
        } else {
            self.x
        };
        let new_y = if self.y.size() < delta {
            self.y.expand(delta)
        } else {
            self.y
        };
        let new_z = if self.z.size() < delta {
            self.z.expand(delta)
        } else {
            self.z
        };
        Self {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn new_point(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    pub fn new_aabb(box0: &Aabb, box1: &Aabb) -> Self {
        Self {
            x: Interval::new_interval(&box0.x, &box1.x),
            y: Interval::new_interval(&box0.y, &box1.y),
            z: Interval::new_interval(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
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
                if t0 < ray_t.min {
                    ray_t.min = t0;
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
#[allow(dead_code)]
pub const EMPTY: Aabb = Aabb {
    x: easy_task::interval::EMPTY,
    y: easy_task::interval::EMPTY,
    z: easy_task::interval::EMPTY,
};
#[allow(dead_code)]
pub const UNIVERSE: Aabb = Aabb {
    x: easy_task::interval::UNIVERSE,
    y: easy_task::interval::UNIVERSE,
    z: easy_task::interval::UNIVERSE,
};
impl std::ops::Add<Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x: &self.x + rhs.x(),
            y: &self.y + rhs.y(),
            z: &self.z + rhs.z(),
        }
    }
}

impl std::ops::Add<&Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: &Aabb) -> Self::Output {
        Aabb {
            x: self.x() + &rhs.x,
            y: self.y() + &rhs.y,
            z: self.z() + &rhs.z,
        }
    }
}

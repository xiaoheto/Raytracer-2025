use crate::easy_task::vec3::{Vec3, cross, unit_vector};

#[derive(Default, Debug, Clone, Copy)]
pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: Vec3) -> Self {
        let ret2 = unit_vector(n);
        let a = if ret2.x() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let ret1 = unit_vector(cross(ret2, a));
        let ret0 = cross(ret2, ret1);

        Self {
            axis: [ret0, ret1, ret2],
        }
    }
    #[allow(dead_code)]
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    #[allow(dead_code)]
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        (v[0] * self.axis[0]) + (v[1] * self.axis[1]) + (v[2] * self.axis[2])
    }
}

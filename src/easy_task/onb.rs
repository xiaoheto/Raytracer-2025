use crate::easy_task::vec3::{Vec3, cross, unit_vector};

#[derive(Debug, Clone, Copy)]
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: Vec3) -> Self {
        let axis2 = unit_vector(n);
        let a = if axis2.x() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let axis1 = unit_vector(cross(axis2, a));
        let axis0 = cross(axis2, axis1);

        Self {
            axis: [axis0, axis1, axis2],
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
        v[0] * self.axis[0] + v[1] * self.axis[1] + v[2] * self.axis[2]
    }
}

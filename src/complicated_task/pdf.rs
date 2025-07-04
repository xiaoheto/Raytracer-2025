use crate::easy_task::onb::Onb;
use crate::easy_task::vec3::{Vec3, dot, random_cosine_direction, random_unit_vector, unit_vector};
use crate::tools::rtweekend::PI;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConsinePdf {
    uvw: Onb,
}
impl ConsinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}
impl Pdf for ConsinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = dot(unit_vector(direction), self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}

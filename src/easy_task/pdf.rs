use super::onb;
use super::rtweekend;
use super::vec3;
use super::vec3::{Point3, Vec3};
use crate::easy_task::hittable::Hittable;
use onb::Onb;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct SpherePdf;

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * rtweekend::PI)
    }

    fn generate(&self) -> Vec3 {
        vec3::random_unit_vector()
    }
}
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    #[allow(dead_code)]
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: Onb::new_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = vec3::dot(vec3::unit_vector(direction), self.uvw.w());
        0.0_f64.max(cosine_theta / rtweekend::PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_v(vec3::random_cosine_direction())
    }
}

#[derive(Clone)]
pub struct HittablePdf {
    pub objects: Arc<dyn Hittable + Send + Sync>,
    pub origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable + Send + Sync>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

pub struct MixturePdf {
    pub p: [Arc<dyn Pdf + Sync + Send>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf + Sync + Send>, p1: Arc<dyn Pdf + Sync + Send>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rtweekend::random_double_range(0.0, 1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}

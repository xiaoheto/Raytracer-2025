use super::aabb::Aabb;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::interval::{self, Interval};
use super::material::{Isotropic, Material};
use super::ray::Ray;
use super::rtweekend;
use super::texture::Texture;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Sync + Send>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material + Sync + Send>,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn new(
        boundary: Arc<dyn Hittable + Sync + Send>,
        density: f64,
        a: Arc<dyn Texture + Sync + Send>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }
    #[allow(dead_code)]
    pub fn new_with_color(
        boundary: Arc<dyn Hittable + Sync + Send>,
        density: f64,
        albedo: Color,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_with_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && rtweekend::random_double() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, &interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(
            r,
            &Interval::new(rec1.t + 0.0001, rtweekend::INFINITY),
            &mut rec2,
        ) {
            return false;
        }

        if debugging {
            eprintln!("\nray_tmin={} ray_tmax={}", rec1.t, rec2.t);
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rtweekend::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(Arc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}

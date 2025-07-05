use crate::easy_task::aabb::Aabb;
use crate::easy_task::color::Color;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::material::{Isotropic, Material};
use crate::easy_task::ray::Ray;
use crate::easy_task::texture::Texture;
use crate::easy_task::vec3::Vec3;
use crate::tools::rtweekend::{INFINITY, random_double};
use std::sync::Arc;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundry: Arc<dyn Hittable + Send + Sync>,
    pub neg_inv_density: f64,
    pub phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn new_texture(
        boundry: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        tex: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            boundry,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    #[allow(dead_code)]
    pub fn new_color(boundry: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundry,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        let temp_universe = Interval::new(-INFINITY, INFINITY);
        if !self.boundry.hit(r, &temp_universe, &mut rec1) {
            return false;
        }

        if !self
            .boundry
            .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
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
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(Arc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundry.bounding_box()
    }
}

use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3;
use crate::easy_task::vec3::{Point3, Vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat,
        }
    }

    pub fn new_move(center1: Point3, center2: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().squared_length();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminate = h * h - a * c;
        if discriminate < 0.0 {
            return false;
        }

        let sqrtd = discriminate.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(Rc::clone(&self.mat));
        true
    }
}

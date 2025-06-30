use crate::easy_task::aabb::Aabb;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3;
use crate::easy_task::vec3::{Point3, Vec3};
use crate::tools::rtweekend::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let center = Ray::new(static_center, Vec3::default());
        let r = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);

        Self {
            center,
            radius: r,
            mat,
            bbox: Aabb::new_point(static_center - rvec, static_center + rvec),
        }
    }

    pub fn new_move(center1: Point3, center2: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let r = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_point(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::new_point(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Self {
            center,
            radius: r,
            mat,
            bbox: Aabb::new_aabb(box1, box2),
        }
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
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
        (rec.u, rec.v) = self.get_sphere_uv(outward_normal);
        rec.mat = Some(Rc::clone(&self.mat));
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

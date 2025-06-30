use crate::easy_task::aabb::Aabb;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::rc::Rc;

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::new_point(q, q + u + v);
        let bbox_diagonal2 = Aabb::new_point(q + u, q + v);
        let bbox = Aabb::new_aabb(bbox_diagonal1, bbox_diagonal2);
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        Self {
            q,
            u,
            v,
            w: n / dot(n, n),
            mat,
            bbox,
            normal,
            d,
        }
    }
    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}
impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }
}

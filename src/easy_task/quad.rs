use crate::easy_task::aabb::Aabb;
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::rtweekend::random_double;
use crate::easy_task::vec3::{Point3, Vec3, cross, dot, unit_vector};
use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Quad {
    pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;

        true
    }

    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let bbox_diagonal1 = Aabb::new_point(&q, &(q + u + v));
        let bbox_diagonal2 = Aabb::new_point(&(q + u), &(q + v));
        let bbox = Aabb::new_aabb(&bbox_diagonal1, &bbox_diagonal2);
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);
        let area = n.length();
        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
            area,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = r.at(t) - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction),
            &Interval::new(0.0001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (dot(direction, rec.normal) / direction.length()).abs();

        distance_squared / (cosine * self.area)
    }

    #[allow(dead_code)]
    fn random(&self, origin: Point3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - origin
    }
}

pub fn box_(a: Point3, b: Point3, mat: Arc<dyn Material + Sync + Send>) -> Arc<HittableList> {
    let mut sides = HittableList::default();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        Arc::clone(&mat),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        Arc::clone(&mat),
    )));

    Arc::new(sides)
}

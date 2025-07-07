use crate::easy_task::aabb::Aabb;
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::rtweekend::{INFINITY, PI, degrees_to_radians};
use crate::easy_task::vec3::{Point3, Vec3, dot};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,    //交点
    pub normal: Vec3, //法线
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material + Send + Sync>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &Aabb;
}

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
    bbox: Aabb,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_point(&(static_center - rvec), &(static_center + rvec));
        Self {
            center: Ray::new(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius,
            mat,
            bbox,
        }
    }

    #[allow(dead_code)]
    pub fn new_move(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let radius = radius.max(0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let center = Ray::new(center1, center2 - center1);
        let box1 = Aabb::new_point(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let box2 = Aabb::new_point(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));

        Self {
            center,
            radius,
            mat,
            bbox: Aabb::new_aabb(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
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
        (rec.u, rec.v) = Self::get_sphere_uv(outward_normal);
        rec.mat = Some(Arc::clone(&self.mat));

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable + Sync + Send>,
    offset: Vec3,
    bbox: Aabb,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable + Sync + Send>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Arc<dyn Hittable + Sync + Send>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 将光线从世界空间变换到对象空间
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );
        let direction = Point3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotated_r = Ray::new_time(origin, direction, r.time());

        // 在对象空间中确定是否存在交点（如果有，确定在哪里）
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // 将交点从对象空间变换到世界空间
        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
        rec.p = p;

        // 将法线从对象空间变换到世界空间
        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
        rec.normal = normal;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    (0..3).for_each(|c| {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    })
                })
            })
        });

        let bbox = Aabb::new_point(&min, &max);
        Self {
            object: p,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

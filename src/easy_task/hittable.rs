use crate::easy_task::aabb::Aabb;
use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3, dot};
use crate::tools::rtweekend::{INFINITY, degrees_to_radians};
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,    //交点位置
    pub normal: Vec3, // 交点法向量
    pub t: f64,       //距离
    pub front_face: bool,
    pub mat: Option<Rc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

#[derive(Clone)]
pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = offset + object.bounding_box();
        Self {
            object,
            offset,
            bbox,
        }
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, -INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        (0..2).for_each(|i| {
            {
                (0..2).for_each(|j| {
                    {
                        (0..2).for_each(|k| {
                            let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                            let y = j as f64 * bbox.y.max + (1.0 - i as f64) * bbox.y.min;
                            let z = k as f64 * bbox.z.max + (1.0 - i as f64) * bbox.z.min;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        })
                    }
                })
            }
        });

        bbox = Aabb::new_point(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_time(origin, direction, r.time());

        if !self.object.hit(rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

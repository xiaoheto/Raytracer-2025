use crate::easy_task::interval::Interval;
use crate::easy_task::material::Material;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3, dot};
use std::rc::Rc;
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,    //交点位置
    pub normal: Vec3, // 交点法向量
    pub t: f64,       //距离
    pub front_face: bool,
    pub mat: Option<Rc<dyn Material>>,
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
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

use crate::easy_task::ray::Ray;
use crate::easy_task::vec3::{Point3, Vec3, dot};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point3,    //交点位置
    pub normal: Vec3, // 交点法向量
    pub t: f64,       //距离
    pub front_face: bool,
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
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use crate::easy_task::onb::Onb;
use crate::easy_task::rtweekend::{PI, random_double};
use crate::easy_task::texture::{SolidColor, Texture};
use crate::easy_task::vec3::{
    Point3, dot, random_cosine_direction, random_unit_vector, reflect, refract, unit_vector,
};
use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    #[allow(dead_code)]
    pub fn new_texture(a: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { tex: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let uvw = Onb::new_from_w(rec.normal);
        let scatter_direction = uvw.local_v(random_cosine_direction());

        *scattered = Ray::new_time(rec.p, unit_vector(scatter_direction), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        *pdf = dot(uvw.w(), scattered.direction()) / PI;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
    }
}

pub struct Metal {
    pub albedo: Color,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let mut reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new_time(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0); // 或者其他合适的初始值
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray::new_time(rec.p, direction, r_in.time());
        true
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { tex }
    }

    pub fn new_color(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if !rec.front_face {
            Color::new(0.0, 0.0, 0.0)
        } else {
            self.tex.value(u, v, p)
        }
    }
}

pub struct Isotropic {
    pub tex: Arc<dyn Texture + Sync + Send>,
}

impl Isotropic {
    pub fn new(a: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { tex: a }
    }

    #[allow(dead_code)]
    pub fn new_with_color(c: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new_time(rec.p, random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        *pdf = 1.0 / (4.0 * PI);
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

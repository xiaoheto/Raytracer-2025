use crate::easy_task::color::Color;
use crate::easy_task::hittable::HitRecord;
use crate::easy_task::ray::Ray;
use crate::easy_task::texture::{SolidColor, Texture};
use crate::easy_task::vec3::{
    Point3, Vec3, dot, random_on_hemisphere, random_unit_vector, reflect, refract, unit_vector,
};
use crate::tools::rtweekend::{PI, random_double};
use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        true
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _r_ec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(a)),
        }
    }

    #[allow(dead_code)]
    pub fn new_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = random_on_hemisphere(rec.normal);
        *scattered = Ray::new_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(rec.normal, unit_vector(scattered.direction()));
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new_time(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let mut _direction = Vec3::default();

        if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            _direction = reflect(unit_direction, rec.normal);
        } else {
            _direction = refract(unit_direction, rec.normal, ri);
        }
        *scattered = Ray::new_time(rec.p, _direction, r_in.time());
        true
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: Ray,
        _rec: HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_color(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Isotropic {
    #[allow(dead_code)]
    pub fn new(a: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo: a }
    }

    #[allow(dead_code)]
    pub fn new_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new_time(rec.p, random_unit_vector(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

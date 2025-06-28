use crate::easy_task::color::Color;
use crate::easy_task::hittable::HitRecord;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3;
use crate::easy_task::vec3::{dot, random_unit_vector, refract, unit_vector};

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
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
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
        let mut reflected = vec3::reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
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
        let refracted = refract(unit_direction, rec.normal, ri);

        *scattered = Ray::new(rec.p, refracted);
        true
    }
}

use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use crate::easy_task::pdf::{CosinePdf, Pdf, SpherePdf};
use crate::easy_task::rtweekend::{PI, random_double};
use crate::easy_task::texture::{SolidColor, Texture};
use crate::easy_task::vec3::{
    Point3, Vec3, dot, random_in_unit_disk, reflect, refract, unit_vector,
};
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf = Box::new(CosinePdf::new(rec.normal));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, unit_vector(scattered.direction()));
        if cosine < 0.0 { 0.0 } else { cosine / PI }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo;
        srec.skip_pdf = true;
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        srec.skip_pdf_ray = Ray::new_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_disk(),
            r_in.time(),
        );
        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.skip_pdf = true;

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

        srec.skip_pdf_ray = Ray::new_time(rec.p, direction, r_in.time());
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
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, rec.p);
        srec.pdf = Box::new(SpherePdf {});
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}

pub struct NonePdf;

impl Pdf for NonePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Box<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf: Box::new(NonePdf {}),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

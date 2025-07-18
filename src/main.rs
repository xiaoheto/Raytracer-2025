mod easy_task;
use crate::easy_task::camera::Camera;
use crate::easy_task::color::Color;
use crate::easy_task::hittable::{Hittable, RotateY, Sphere, Translate};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::material::{Dielectric, DiffuseLight, Lambertian, Material};
use crate::easy_task::quad::{Quad, box_};
use crate::easy_task::vec3::{Point3, Vec3};
use std::sync::Arc;

fn cornell_box() {
    let mut world = HittableList::default();

    let red: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0),
        Vec3::new(130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Arc::clone(&white),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Sync + Send> = box_(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white,
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let mut lights = HittableList::default();
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        light,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(Arc::new(world), Arc::new(lights));
}

fn main() {
    cornell_box()
}

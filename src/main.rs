mod easy_task;
mod tools;
use std::rc::Rc;

use crate::easy_task::camera::Camera;
use crate::easy_task::color::Color;
use crate::easy_task::material;
use crate::easy_task::material::Dielectric;
use crate::easy_task::sphere::Sphere;
use easy_task::hittable_list::HittableList;
use easy_task::vec3::Point3;

fn main() {
    // World
    let mut world = HittableList::default();
    let material_ground = Rc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.00 / 1.30));
    let material_right = Rc::new(material::Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    // Render
    cam.render(&world);
}

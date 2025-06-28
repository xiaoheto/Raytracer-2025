mod easy_task;
mod tools;
use std::rc::Rc;

use crate::easy_task::camera::Camera;
use crate::easy_task::color::Color;
use crate::easy_task::material::Lambertian;
use crate::easy_task::sphere::Sphere;
use crate::tools::rtweekend::PI;
use easy_task::hittable_list::HittableList;
use easy_task::vec3::Point3;

fn main() {
    // World
    let mut world = HittableList::default();

    let r = (PI / 4.0).cos();
    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    // Render
    cam.render(&world);
}

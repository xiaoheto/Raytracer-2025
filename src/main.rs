mod easy_task;
mod tools;
use std::rc::Rc;

use crate::easy_task::camera::Camera;
use crate::easy_task::sphere::Sphere;
use easy_task::hittable_list::HittableList;
use easy_task::vec3::Point3;

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    // Render
    cam.render(&world);
}

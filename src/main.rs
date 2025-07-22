mod easy_task;
use crate::easy_task::bvh_node::BvhNode;
use crate::easy_task::camera::Camera;
use crate::easy_task::color::Color;
use crate::easy_task::constant_medium::ConstantMedium;
use crate::easy_task::hittable::{Hittable, RotateY, Sphere, Translate};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::material::{Dielectric, DiffuseLight, Lambertian, Material};
use crate::easy_task::quad::{Quad, box_};
use crate::easy_task::rtweekend::random_double_range;
use crate::easy_task::texture::{ImageTexture, Texture};
use crate::easy_task::vec3::{Point3, Vec3, random_range};
use std::sync::Arc;
#[allow(dead_code)]
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
    final_scene(800, 1000, 40)
}
#[allow(dead_code)]
fn earth() {
    let earth_texture: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("zym.jpg"));
    let earth_surface: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new_texture(Arc::clone(&earth_texture)));
    let mut globe: Arc<dyn Hittable + Sync + Send> =
        Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    globe = Arc::new(RotateY::new(globe, -90.0));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));
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

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 10;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(globe, Arc::new(lights));
}
fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::default();
    let ground: Arc<dyn Material + Send + Sync> =
        Arc::new(Lambertian::new(Color::new(0.9, 0.9, 0.9)));

    let boxes_per_side = 20;
    (0..boxes_per_side).for_each(|i| {
        (0..boxes_per_side).for_each(|j| {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(box_(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            ));
        });
    });

    let mut world = HittableList::default();

    world.add(Arc::new(BvhNode::new_list(&mut boxes1)));

    let light: Arc<dyn Material + Send + Sync> =
        Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let mut lights = HittableList::default();
    lights.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<dyn Material + Send + Sync> =
        Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_move(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    let mmat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian::new_texture(Arc::new(
        ImageTexture::new("moon.jpg"),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        mmat,
    )));

    let zmat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian::new_texture(Arc::new(
        ImageTexture::new("zym.jpg"),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        zmat,
    )));

    let boundary: Arc<dyn Hittable + Sync + Send> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::clone(&boundary));
    world.add(Arc::new(ConstantMedium::new_with_color(
        Arc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary: Arc<dyn Hittable + Sync + Send> = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        Arc::clone(&boundary),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian::new_texture(Arc::new(
        ImageTexture::new("earthmap.jpg"),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let jmat: Arc<dyn Material + Send + Sync> = Arc::new(Lambertian::new_texture(Arc::new(
        ImageTexture::new("Jupiter.jpg"),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        jmat,
    )));

    let mut boxes2 = HittableList::default();
    let white: Arc<dyn Material + Send + Sync> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    (0..ns).for_each(|_| {
        boxes2.add(Arc::new(Sphere::new(
            random_range(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    });

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new_list(&mut boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width as i32;
    cam.samples_per_pixel = samples_per_pixel as i32;
    cam.max_depth = max_depth as i32;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let world: Arc<dyn Hittable + Send + Sync> = Arc::new(world);

    cam.render(world, Arc::new(lights));
}

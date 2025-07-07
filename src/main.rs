mod easy_task;
use crate::easy_task::camera::Camera;
use crate::easy_task::color::Color;
use crate::easy_task::hittable::{RotateY, Translate};
use crate::easy_task::hittable_list::HittableList;
use crate::easy_task::material::{DiffuseLight, Lambertian, Material};
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
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
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

    let box1 = box_(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = box_(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(Arc::new(world));
}

fn main() {
    cornell_box()
}

// fn cornell_smoke() {
//     let mut world = HittableList::default();
//
//     let red: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
//     let white: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
//     let green: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
//     let light: Arc<dyn Material + Sync + Send> = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
//
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(555.0, 0.0, 0.0),
//             Vec3::new(0.0, 555.0, 0.0),
//             Vec3::new(0.0, 0.0, 555.0),
//             green
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(0.0, 0.0, 0.0),
//             Vec3::new(0.0, 555.0, 0.0),
//             Vec3::new(0.0, 0.0, 555.0),
//             red
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(113.0, 554.0, 127.0),
//             Vec3::new(330.0, 0.0, 0.0),
//             Vec3::new(0.0, 0.0, 305.0),
//             light
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(0.0, 555.0, 0.0),
//             Vec3::new(555.0, 0.0, 0.0),
//             Vec3::new(0.0, 0.0, 555.0),
//             Arc::clone(&white)
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(0.0, 0.0, 0.0),
//             Vec3::new(555.0, 0.0, 0.0),
//             Vec3::new(0.0, 0.0, 555.0),
//             Arc::clone(&white)
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(0.0, 0.0, 555.0),
//             Vec3::new(555.0, 0.0, 0.0),
//             Vec3::new(0.0, 0.0, 555.0),
//             Arc::clone(&white)
//         )
//     ));
//     world.add(Arc::new(
//         Quad::new(
//             Point3::new(0.0, 0.0, 555.0),
//             Vec3::new(555.0, 0.0, 0.0),
//             Vec3::new(0.0, 555.0, 0.0),
//             Arc::clone(&white)
//         )
//     ));
//
//     let box1 = box_(
//         Point3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 330.0, 165.0),
//         Arc::clone(&white)
//     );
//     let box1 = Arc::new(RotateY::new(box1, 15.0));
//     let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
//
//     let box2 = box_(
//         Point3::new(0.0, 0.0, 0.0),
//         Vec3::new(165.0, 165.0, 165.0),
//         Arc::clone(&white)
//     );
//     let box2 = Arc::new(RotateY::new(box2, -18.0));
//     let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
//
//     world.add(Arc::new(
//         ConstantMedium::new_with_color(box1, 0.01, Color::new(0.0, 0.0, 0.0))
//     ));
//     world.add(Arc::new(
//         ConstantMedium::new_with_color(box2, 0.01, Color::new(1.0, 1.0, 1.0))
//     ));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 1.0;
//     cam.image_width = 600;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 20;
//     cam.background = Color::default();
//
//     cam.vfov = 40.0;
//     cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
//     cam.lookat = Point3::new(278.0, 278.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(Arc::new(world));
// }
// fn earth() {
//     let earth_texture: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("earthmap.jpg"));
//     let earth_surface: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new_texture(Arc::clone(&earth_texture)));
//     let globe: Arc<dyn Hittable + Send + Sync> =
//         Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 50;
//     cam.max_depth = 10;
//     cam.background = Color::new(0.7, 0.8, 1.0);
//
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(globe);
// }
//
// fn perlin_spheres() {
//     let mut world = HittableList::default();
//
//     let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new_texture(Arc::clone(&pertext))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new_texture(Arc::clone(&pertext))),
//     )));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 50;
//     cam.max_depth = 10;
//     cam.background = Color::new(0.7, 0.8, 1.0);
//
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(Arc::new(world));
// }
//
// fn quads() {
//     let mut world = HittableList::default();
//
//     // Material
//     let left_red: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
//     let back_green: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
//     let right_blue: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
//     let upper_orange: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
//     let lower_teal: Arc<dyn Material + Sync + Send> =
//         Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));
//
//     // Quad
//     world.add(Arc::new(Quad::new(
//         Point3::new(-3.0, -2.0, 5.0),
//         Vec3::new(0.0, 0.0, -4.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         left_red,
//     )));
//     world.add(Arc::new(Quad::new(
//         Point3::new(-2.0, -2.0, 0.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         back_green,
//     )));
//     world.add(Arc::new(Quad::new(
//         Point3::new(3.0, -2.0, 1.0),
//         Vec3::new(0.0, 0.0, 4.0),
//         Vec3::new(0.0, 4.0, 0.0),
//         right_blue,
//     )));
//     world.add(Arc::new(Quad::new(
//         Point3::new(-2.0, 3.0, 1.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, 4.0),
//         upper_orange,
//     )));
//     world.add(Arc::new(Quad::new(
//         Point3::new(-2.0, -3.0, 5.0),
//         Vec3::new(4.0, 0.0, 0.0),
//         Vec3::new(0.0, 0.0, -4.0),
//         lower_teal,
//     )));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 1.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 50;
//     cam.background = Color::new(0.7, 0.8, 1.0);
//
//     cam.vfov = 80.0;
//     cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(Arc::new(world));
// }
//
// fn simple_light() {
//     let mut world = HittableList::default();
//
//     let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new(4.0));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new_texture(Arc::clone(&pertext))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new_texture(pertext)),
//     )));
//
//     let difflight: Arc<dyn Material + Send + Sync> =
//         Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 7.0, 0.0),
//         2.0,
//         difflight.clone(),
//     )));
//     world.add(Arc::new(Quad::new(
//         Point3::new(3.0, 1.0, -2.0),
//         Vec3::new(2.0, 0.0, 0.0),
//         Vec3::new(0.0, 2.0, 0.0),
//         difflight,
//     )));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 50;
//     cam.background = Color::default();
//
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
//     cam.lookat = Point3::new(0.0, 2.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(Arc::new(world));
// }
//
// fn bouncing_spheres() {
//     // World
//     let mut world = HittableList::default();
//
//     let checker = Arc::new(CheckerTexture::new_color(
//         0.32,
//         Color::new(0.2, 0.3, 0.1),
//         Color::new(0.9, 0.9, 0.9),
//     ));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new_texture(checker)),
//     )));
//
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_double();
//             let center = Point3::new(
//                 a as f64 + 0.9 * random_double(),
//                 0.2,
//                 b as f64 + 0.9 * random_double(),
//             );
//
//             if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 let sphere_material: Arc<dyn Material + Sync + Send> = if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = random() * random();
//                     Arc::new(Lambertian::new(albedo))
//                 } else if choose_mat < 0.95 {
//                     // metal
//                     let albedo = random_range(0.5, 1.0);
//                     let fuzz = random_double_range(0.0, 0.5);
//                     Arc::new(Metal::new(albedo, fuzz))
//                 } else {
//                     // glass
//                     Arc::new(Dielectric::new(1.5))
//                 };
//
//                 let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
//                 world.add(Arc::new(Sphere::new_move(
//                     center,
//                     center2,
//                     0.2,
//                     sphere_material,
//                 )));
//             }
//         }
//     }
//
//     let material1 = Arc::new(Dielectric::new(1.5));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 1.0, 0.0),
//         1.0,
//         material1,
//     )));
//
//     let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(-4.0, 1.0, 0.0),
//         1.0,
//         material2,
//     )));
//
//     let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(4.0, 1.0, 0.0),
//         1.0,
//         material3,
//     )));
//
//     world = HittableList::new(Arc::new(BvhNode::new_list(&mut world)));
//
//     // Camera
//     let mut cam = Camera::default();
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 50;
//     cam.background = Color::new(0.7, 0.8, 1.0);
//
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.6;
//     cam.focus_dist = 10.0;
//
//     let world = Arc::new(world);
//     cam.render(world);
// }
//
// fn cherkered_spheres() {
//     let mut world = HittableList::default();
//
//     let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new_color(
//         0.32,
//         Color::new(0.2, 0.3, 0.1),
//         Color::new(0.9, 0.9, 0.9),
//     ));
//
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, -10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new_texture(Arc::clone(&checker))),
//     )));
//     world.add(Arc::new(Sphere::new(
//         Point3::new(0.0, 10.0, 0.0),
//         10.0,
//         Arc::new(Lambertian::new_texture(Arc::clone(&checker))),
//     )));
//
//     let mut cam = Camera::default();
//
//     cam.aspect_ratio = 16.0 / 9.0;
//     cam.image_width = 400;
//     cam.samples_per_pixel = 100;
//     cam.max_depth = 50;
//     cam.background = Color::new(0.7, 0.8, 1.0);
//
//     cam.vfov = 20.0;
//     cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
//     cam.lookat = Point3::new(0.0, 0.0, 0.0);
//     cam.vup = Vec3::new(0.0, 1.0, 0.0);
//
//     cam.defocus_angle = 0.0;
//
//     cam.render(Arc::new(world));
// }

mod easy_task;

use crate::easy_task::vec3::unit_vector;
use easy_task::color::Color;
use easy_task::ray::Ray;
use easy_task::vec3;
use easy_task::vec3::Point3;
use easy_task::vec3::Vec3;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

//判断光线是否与球体相交
fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.origin();
    let a = vec3::dot(r.direction(), r.direction());
    let b = -2.0 * vec3::dot(r.direction(), oc);
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let path = "output/book1/image4.ppm";
    let dir_path = std::path::Path::new("output/book1"); // 创建 Path 对象
    if !dir_path.exists() {
        match create_dir_all(dir_path) {
            Ok(_) => println!("Directory 'output/book1' created successfully"),
            Err(e) => {
                eprintln!("Failed to create directory: {}", e);
                panic!("Failed to create directory: {}", e);
            }
        }
    }

    let mut file = File::create(path).expect("Failed to create file");
    // 写入 PPM 文件头
    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", image_width, image_height).unwrap();
    writeln!(file, "255").unwrap();

    for j in 0..image_height {
        println!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            let r_val = (pixel_color.x() * 255.999) as u8;
            let g_val = (pixel_color.y() * 255.999) as u8;
            let b_val = (pixel_color.z() * 255.999) as u8;

            // 将像素的 RGB 值写入文件
            writeln!(file, "{} {} {}", r_val, g_val, b_val).unwrap();
        }
    }

    println!("\nImage saved as \"{}\"", path);
}

use crate::easy_task::color::{Color, linear_to_gamma};
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::ray::Ray;
use crate::easy_task::vec3;
use crate::easy_task::vec3::{Point3, Vec3, random_in_unit_disk};
use crate::tools::rtweekend;
use crate::tools::rtweekend::{degrees_to_radians, random_double};
use std::fs::{File, create_dir_all};
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_samples_scale: 0.1,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    fn ray_color(r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth < 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if world.hit(r, &mut Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if let Some(mat) = rec.mat.clone() {
                if mat.scatter(r, rec, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(scattered, depth - 1, world);
                }
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let path = "output/book2/image7.ppm";
        let dir_path = std::path::Path::new("output/book2"); // 创建 Path 对象
        if !dir_path.exists() {
            match create_dir_all(dir_path) {
                Ok(_) => println!("Directory 'output/book2' created successfully"),
                Err(e) => {
                    eprintln!("Failed to create directory: {}", e);
                    panic!("Failed to create directory: {}", e);
                }
            }
        }

        let mut file = File::create(path).expect("Failed to create file");
        // 写入 PPM 文件头
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.image_width, self.image_height).unwrap();
        writeln!(file, "255").unwrap();

        for j in 0..self.image_height {
            println!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(r, self.max_depth, world);
                }

                pixel_color *= self.pixel_samples_scale;

                let mut r = pixel_color.x();
                let mut g = pixel_color.y();
                let mut b = pixel_color.z();

                r = linear_to_gamma(r);
                g = linear_to_gamma(g);
                b = linear_to_gamma(b);

                let intensity = Interval::new(0.000, 0.999);
                let r_val = (256.0 * intensity.clamp(r)) as i32;
                let g_val = (256.0 * intensity.clamp(g)) as i32;
                let b_val = (256.0 * intensity.clamp(b)) as i32;

                // 将像素的 RGB 值写入文件
                writeln!(file, "{} {} {}", r_val, g_val, b_val).unwrap();
            }
        }

        println!("\nImage saved as \"{}\"", path);
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // 确定视口尺寸。
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // 计算相机坐标系的 u,v,w 单位基向量。
        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素的位置。
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset: Vec3 = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_time(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}

use crate::easy_task::color::{Color, linear_to_gamma};
use crate::easy_task::hittable::{HitRecord, Hittable};
use crate::easy_task::interval::Interval;
use crate::easy_task::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use crate::easy_task::ray::Ray;
use crate::easy_task::rtweekend::{INFINITY, degrees_to_radians, random_double};
use crate::easy_task::vec3::{Point3, Vec3, cross, random_in_unit_disk, unit_vector};
use crossbeam::channel;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,   // 相机查看的位置
    pub lookat: Point3,     // 相机在看的点
    pub vup: Vec3,          // 指定的向上方向
    pub defocus_angle: f64, // 通过每个像素的光线的变化角度
    pub focus_dist: f64,    // 从相机观察点到完美对焦平面的距离
    pub background: Color,

    image_height: i32,
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
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
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            background: Color::default(),

            image_height: 0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
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
    fn ray_color(
        &self,
        r: &Ray,
        depth: i32,
        world: &Arc<dyn Hittable + Send + Sync>,
        lights: &Arc<dyn Hittable + Send + Sync>,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if !world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        if let Some(mat) = rec.mat.clone() {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            let mut pdf_value = 0.0;
            let color_from_emission = mat.emitted(r, &rec, rec.u, rec.v, rec.p);
            if !mat.scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf_value) {
                return color_from_emission;
            }

            let p0: Arc<dyn Pdf + Sync + Send> = Arc::new(HittablePdf::new(lights.clone(), rec.p));
            let p1: Arc<dyn Pdf + Sync + Send> = Arc::new(CosinePdf::new(rec.normal));
            let mixed_pdf = MixturePdf::new(p0, p1);

            scattered = Ray::new_time(rec.p, mixed_pdf.generate(), r.time());
            pdf_value = mixed_pdf.value(scattered.direction());

            let scattering_pdf = mat.scattering_pdf(r, &rec, &scattered);

            let sample_color = self.ray_color(&scattered, depth - 1, world, lights);
            let color_from_scatter = (attenuation * scattering_pdf * sample_color) / pdf_value;

            color_from_emission + color_from_scatter
        } else {
            Color::default()
        }
    }
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp as f64 * self.sqrt_spp as f64);
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素的位置。
        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(
        &mut self,
        world: Arc<dyn Hittable + Send + Sync>,
        lights: Arc<dyn Hittable + Send + Sync>,
    ) {
        self.initialize();

        let path = "output/book3/image9.ppm";
        let dir_path = std::path::Path::new("output/book3");
        if !dir_path.exists() {
            create_dir_all(dir_path).expect("Failed to create directory");
        }
        let mut file = File::create(path).expect("Failed to create file");

        // 写入 PPM 文件头
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.image_width, self.image_height).unwrap();
        writeln!(file, "255").unwrap();

        let (tx, rx) = channel::unbounded();
        let world = Arc::clone(&world);
        let lights = Arc::clone(&lights);

        let image_width = self.image_width;
        let pixel_samples_scale = self.pixel_samples_scale;
        let camera = *self;
        let max_depth = self.max_depth;
        let sqrt_spp = self.sqrt_spp;

        crossbeam::scope(|scope| {
            for j in 0..self.image_height {
                let tx = tx.clone();
                let world = Arc::clone(&world);
                let lights = Arc::clone(&lights);
                scope.spawn(move |_| {
                    let mut row = String::new();
                    for i in 0..image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for s_j in 0..sqrt_spp {
                            for s_i in 0..sqrt_spp {
                                let r = camera.get_ray(i, j, s_i, s_j);
                                pixel_color += camera.ray_color(&r, max_depth, &world, &lights);
                            }
                        }

                        pixel_color *= pixel_samples_scale; // pixel_samples_scale = 1.0 / samples_per_pixel
                        let mut r = pixel_color.x();
                        let mut g = pixel_color.y();
                        let mut b = pixel_color.z();

                        r = linear_to_gamma(r);
                        g = linear_to_gamma(g);
                        b = linear_to_gamma(b);

                        let intensity = Interval::new(0.0, 0.999);

                        let rbyte = (256.0 * intensity.clamp(r)) as i32;
                        let gbyte = (256.0 * intensity.clamp(g)) as i32;
                        let bbyte = (256.0 * intensity.clamp(b)) as i32;
                        row += &format!("{} {} {} ", rbyte, gbyte, bbyte);
                    }
                    tx.send((j, row)).expect("send failed");
                });
            }
            drop(tx);
        })
        .unwrap();

        let mut rows: Vec<(i32, String)> = rx.iter().collect();
        rows.sort_by_key(|k| k.0); // 按照行数排序

        for (_j, line) in rows {
            file.write_all(line.as_bytes()).unwrap(); //  写入每一行像素数据
            writeln!(file).unwrap(); // 在每行的像素数据之后插入换行符
        }

        println!("\nImage saved as \"{}\"", path);
    }

    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        let offset = self.samples_square_stratified(s_i, s_j);
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

    fn samples_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }

    #[allow(dead_code)]
    fn sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the defocus disk.
        let p = random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}

use super::vec3;
use crate::easy_task::interval::Interval;
use std::io::Write;
use vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        writeln!(out, "{}{}{}", rbyte, gbyte, bbyte)
    }
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

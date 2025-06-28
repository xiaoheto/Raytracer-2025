use super::vec3;
use crate::easy_task::interval::Interval;
use std::io::Write;
use vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let intensity = Interval::new(0.000, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        writeln!(out, "{}{}{}", rbyte, gbyte, bbyte)
    }
}

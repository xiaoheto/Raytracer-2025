use super::vec3;
use std::io::Write;
use vec3::Vec3;
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, out: &mut dyn Write, pixel_color: Color) -> std::io::Result<()> {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let rbyte = 255.999 * r;
        let gbyte = 255.999 * g;
        let bbyte = 255.999 * b;

        write!(out, "{}{}{}\n", rbyte, gbyte, bbyte)
    }
}

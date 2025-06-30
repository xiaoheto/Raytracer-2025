use crate::easy_task::vec3::Point3;
use crate::tools::rtweekend::{random_double, random_int};

const POINT_COUNT: usize = 256;
#[derive(Debug, Clone)]
pub struct Perlin {
    randfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut randfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            randfloat.push(random_double());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[(self.perm_x
                        [((i as usize + di) as i32 & 255) as usize]
                        ^ self.perm_y[((j as usize + dj) as i32 & 255) as usize]
                        ^ self.perm_z[((k as usize + dk) as i32 & 255) as usize])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i as i32);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}

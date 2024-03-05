use nalgebra::Vector3;
use rand::prelude::*;

use crate::vector3::Vector3Extensions;

#[derive(Debug, Clone)]
pub struct Perlin {
    random_vector: Vec<Vector3<f64>>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;

        let random_vector: Vec<Vector3<f64>> = (0..point_count).map(|_| Vector3::random_float_range(-1.0..1.0).normalize()).collect();

        let perm_x = Self::perlin_generate_perm(point_count);
        let perm_y = Self::perlin_generate_perm(point_count);
        let perm_z = Self::perlin_generate_perm(point_count);

        Perlin {
            random_vector,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vector3<f64>) -> f64 {
        let u = p.x - (p.x).floor();
        let v = p.y - (p.y).floor();
        let w = p.z - (p.z).floor();

        let i = ((p.x).floor()) as i32;
        let j = ((p.y).floor()) as i32;
        let k = ((p.z).floor()) as i32;

        let mut c = [[[Vector3::zeros(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i+di) & 255) as usize] ^ self.perm_y[((j+dj) & 255) as usize] ^ self.perm_z[((k+dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.random_vector[index];
                }
            }
        }

        Self::trilinear_interpolation(c, u, v, w)
    }

    pub fn turbulance(&self, point: Vector3<f64>, depth: Option<i32>) -> f64 {
        let depth = depth.unwrap_or(7);

        let mut accumulation = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulation += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        accumulation.abs()
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut p = (0..point_count).collect();
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut Vec<usize>) {
        for i in (1..p.len()).rev() {
            let target = Self::random_usize(0, i);

            p.swap(i, target)
        }
    }

    fn trilinear_interpolation(c: [[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        
        let mut accumulate = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector3::new(u-i as f64, v-j as f64, w-k as f64);

                    accumulate += (i as f64 * uu + (1.0-i as f64) * (1.0-uu)) *
                                  (j as f64 * vv + (1.0-j as f64) * (1.0-vv)) *
                                  (k as f64 * ww + (1.0-k as f64) * (1.0-ww)) *
                                  c[i][j][k].dot(&weight_v);
                }
            }
        }

        accumulate
    }

    fn random_usize(min: usize, max: usize) -> usize{
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}
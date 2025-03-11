use nalgebra::{Point3, UnitVector3, Vector3};
use rand::Rng;

use crate::utils::random_vector_generator;

const PERLIN_POINTS_COUNT: usize = 256;

pub struct PerlinNoise {
    perm_x: Box<[usize; PERLIN_POINTS_COUNT]>,
    perm_y: Box<[usize; PERLIN_POINTS_COUNT]>,
    perm_z: Box<[usize; PERLIN_POINTS_COUNT]>,
    rand_vec: Box<[UnitVector3<f64>; PERLIN_POINTS_COUNT]>,
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rand_vec =
            [UnitVector3::new_unchecked(Vector3::new(1.0, 0.0, 0.0)); PERLIN_POINTS_COUNT];
        (0..PERLIN_POINTS_COUNT).for_each(|i| {
            rand_vec[i] = random_vector_generator::random_unit_vector3_in_sphere();
        });
        Self {
            perm_x: Box::new(Self::generate_perm()),
            perm_y: Box::new(Self::generate_perm()),
            perm_z: Box::new(Self::generate_perm()),
            rand_vec: Box::new(rand_vec),
        }
    }

    pub fn noise(&self, pos: &Point3<f64>) -> f64 {
        let mut c = [[[Vector3::zeros(); 2]; 2]; 2];
        let u = pos.x - pos.x.floor();
        let v = pos.y - pos.y.floor();
        let w = pos.z - pos.z.floor();

        let i = pos.x.floor() as i32;
        let j = pos.y.floor() as i32;
        let k = pos.z.floor() as i32;

        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    let i_id = ((i + di) & 255) as u8;
                    let id_x = self.perm_x[i_id as usize];
                    let j_id = ((j + dj) & 255) as u8;
                    let id_y = self.perm_y[j_id as usize];
                    let k_id = ((k + dk) & 255) as u8;
                    let id_z = self.perm_z[k_id as usize];
                    let id = id_x ^ id_y ^ id_z;
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[id].into_inner();
                });
            });
        });

        Self::perlin_interpolation(c, u, v, w)
    }

    /// Returns value of perlin interpolation in range `[-1, 1]`
    fn perlin_interpolation(c: [[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;

        let u_hermitian = u * u * (3.0 - 2.0 * u);
        let v_hermitian = v * v * (3.0 - 2.0 * v);
        let w_hermitian = w * w * (3.0 - 2.0 * w);

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let i_d = i as f64;
                    let j_d = j as f64;
                    let k_d = k as f64;
                    let u_part = i_d * u_hermitian + (1.0 - i_d) * (1.0 - u_hermitian);
                    let v_part = j_d * v_hermitian + (1.0 - j_d) * (1.0 - v_hermitian);
                    let w_part = k_d * w_hermitian + (1.0 - k_d) * (1.0 - w_hermitian);

                    let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);

                    acc += u_part * v_part * w_part * c[i][j][k].dot(&weight_v)
                });
            });
        });
        acc
    }

    fn generate_perm() -> [usize; PERLIN_POINTS_COUNT] {
        let mut arr = [0_usize; PERLIN_POINTS_COUNT];
        (0..PERLIN_POINTS_COUNT).for_each(|i| {
            arr[i] = i;
        });
        Self::permute(&mut arr);
        arr
    }

    fn permute(arr: &mut [usize; PERLIN_POINTS_COUNT]) {
        let mut rng = rand::rng();
        for i in (1..=PERLIN_POINTS_COUNT - 1).rev() {
            let to_swap = rng.random::<u32>() as usize % i;
            arr.swap(to_swap, i);
        }
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}

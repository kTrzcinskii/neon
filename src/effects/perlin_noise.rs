use nalgebra::Point3;
use rand::Rng;

const PERLIN_POINTS_COUNT: usize = 256;

pub struct PerlinNoise {
    perm_x: Box<[usize; PERLIN_POINTS_COUNT]>,
    perm_y: Box<[usize; PERLIN_POINTS_COUNT]>,
    perm_z: Box<[usize; PERLIN_POINTS_COUNT]>,
    rand_f64: Box<[f64; PERLIN_POINTS_COUNT]>,
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut rand_f64 = [0.0; PERLIN_POINTS_COUNT];
        (0..PERLIN_POINTS_COUNT).for_each(|i| {
            rand_f64[i] = rng.random();
        });
        Self {
            perm_x: Box::new(Self::generate_perm()),
            perm_y: Box::new(Self::generate_perm()),
            perm_z: Box::new(Self::generate_perm()),
            rand_f64: Box::new(rand_f64),
        }
    }

    pub fn noise(&self, pos: &Point3<f64>) -> f64 {
        let mut c = [[[0.0; 2]; 2]; 2];
        let u = pos.x - pos.x.floor();
        let v = pos.y - pos.y.floor();
        let w = pos.z - pos.z.floor();

        let u_hermitian = u * u * (3.0 - 2.0 * u);
        let v_hermitian = v * v * (3.0 - 2.0 * v);
        let w_hermitian = w * w * (3.0 - 2.0 * w);

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
                    c[di as usize][dj as usize][dk as usize] = self.rand_f64[id];
                });
            });
        });

        Self::trilinear_interpolation(c, u_hermitian, v_hermitian, w_hermitian)
    }

    fn trilinear_interpolation(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut acc = 0.0;
        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let i_d = i as f64;
                    let j_d = j as f64;
                    let k_d = k as f64;
                    let u_part = i_d * u + (1.0 - i_d) * (1.0 - u);
                    let v_part = j_d * v + (1.0 - j_d) * (1.0 - v);
                    let w_part = k_d * w + (1.0 - k_d) * (1.0 - w);
                    acc += u_part * v_part * w_part * c[i][j][k]
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

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
        let i = ((4.0 * pos.x) as i32 & 255) as usize;
        let j = ((4.0 * pos.y) as i32 & 255) as usize;
        let k = ((4.0 * pos.z) as i32 & 255) as usize;
        let id = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.rand_f64[id]
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

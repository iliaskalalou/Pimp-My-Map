use std::usize;

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct DsquareOpts {
    pub size: u32,
    pub roughness: f64,
}

impl Default for DsquareOpts {
    fn default() -> Self {
        Self {
            size: 9,
            roughness: 0.5,
        }
    }
}

fn wiggler(val: f64, bound: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(-bound..bound);
    val + r
}

fn __diamond_square(grid: &mut Vec<Vec<f64>>, opts: &DsquareOpts) {
    let h = grid.len();

    let mut r = opts.roughness;

    let mut i = h - 1;
    while i > 1 {
        let half = i / 2;
        // Phase du diamant
        for x in (half..h).step_by(i) {
            for y in (half..h).step_by(i) {
                let moyenne = (grid[x - half][y - half]
                    + grid[x - half][y + half]
                    + grid[x + half][y + half]
                    + grid[x + half][y - half])
                    / 4.0;
                grid[x][y] = wiggler(moyenne, r);
            }
        }
        // Phase du carré
        let mut decalage = 0;
        for x in (0..h).step_by(half) {
            if decalage == 0 {
                decalage = half;
            } else {
                decalage = 0;
            }
            for y in (decalage..h).step_by(i) {
                let mut somme = 0.0;
                let mut n = 0;
                if x >= half {
                    somme += grid[x - half][y];
                    n += 1;
                }
                if x + half < h {
                    somme += grid[x + half][y];
                    n += 1;
                }
                if y >= half {
                    somme += grid[x][y - half];
                    n += 1;
                }
                if y + half < h {
                    somme += grid[x][y + half];
                    n += 1;
                }
                grid[x][y] = wiggler(somme / n as f64, r);
            }
        }
        r /= 2.0;
        i = half;
    }
}

fn normalize(grid: &mut [Vec<f64>], opts: &DsquareOpts) {
    let d = opts.roughness;
    for l in grid.iter_mut() {
        for val in l.iter_mut() {
            *val = (d - *val).abs() / (2.0 * d);
        }
    }
}

pub fn diamond_square(
    opts: &DsquareOpts,
    base: &Vec<Vec<f64>>,
) -> Vec<Vec<f64>> {
    let l = (2_i32.pow(opts.size) + 1) as usize;
    let mut grid = vec![vec![0.0; l]; l];

    for x in 0..base.len() {
        for y in 0..base[x].len() {
            grid[x][y] = base[x][y];
        }
    }

    __diamond_square(&mut grid, opts);
    normalize(&mut grid, opts);
    grid
}

use rand::distributions::{Distribution, Uniform};

#[derive(Clone, Debug)]
pub struct PerlinOpts {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub base: f64,
    pub permutations: [usize; 512],
    pub octaves: usize,
    pub fallout: f64,
    pub lacunarity: f64,
}

impl Default for PerlinOpts {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            base: 0.0,
            permutations: [0; 512],
            octaves: 8,
            fallout: 0.5,
            lacunarity: 2.0,
        }
    }
}

fn lerp(a: f64, b: f64, w: f64) -> f64 {
    (1.0 - w) * a + b * w
}

fn fade(t: f64) -> f64 {
    // (3.0 - t * 2.0) * t * t // smooth
    (t * (t * 6.0 - 15.0) + 10.0) * t * t * t // smoother
}

pub fn random_permutations() -> [usize; 512] {
    let mut perm = [0; 512];
    let mut rng = rand::thread_rng();

    for i in 0..256 {
        perm[i] = i;
    }

    for i in 0..256 {
        // let j = rng.gen_range(0..256) & 0xFF;
        let j = Uniform::from(0..256).sample(&mut rng) & 0xFF;
        perm.swap(j, i);
    }

    for i in 0..256 {
        perm[i + 256] = perm[i];
    }

    perm
}

fn grad2d(hash: usize, x: f64, y: f64) -> f64 {
    let v = if hash & 1 == 0 { x } else { y };

    if (hash & 2) == 0 {
        -v
    } else {
        v
    }
}

fn grad3d(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else {
        if h == 12 || h == 14 {
            x
        } else {
            z
        }
    };

    let a = if (h & 1) == 0 { u } else { -u };

    a + if (h & 2) == 0 { v } else { -v }
}

fn noise2d(mut x: f64, mut y: f64, perm: [usize; 512]) -> f64 {
    let x0 = (x.floor() as usize) & 255;
    let y0 = (y.floor() as usize) & 255;

    x -= x.floor();
    y -= y.floor();

    let fx = fade(x);
    let fy = fade(y);
    let p0 = perm[x0] + y0;
    let p1 = perm[x0 + 1] + y0;

    lerp(
        lerp(grad2d(perm[p0], x, y), grad2d(perm[p1], x - 1.0, y), fx),
        lerp(
            grad2d(perm[p0 + 1], x, y - 1.0),
            grad2d(perm[p1 + 1], x - 1.0, y - 1.0),
            fx,
        ),
        fy,
    )
}

fn noise3d(mut x: f64, mut y: f64, mut z: f64, perm: [usize; 512]) -> f64 {
    let x0 = (x.floor() as usize) & 255;
    let y0 = (y.floor() as usize) & 255;
    let z0 = (z.floor() as usize) & 255;

    x -= x.floor();
    y -= y.floor();
    z -= z.floor();

    let fx = fade(x);
    let fy = fade(y);
    let fz = fade(z);

    let p0 = perm[x0] + y0;
    let p00 = perm[p0] + z0;
    let p01 = perm[p0 + 1] + z0;
    let p1 = perm[x0 + 1] + y0;
    let p10 = perm[p1] + z0;
    let p11 = perm[p1 + 1] + z0;

    lerp(
        lerp(
            lerp(
                grad3d(perm[p00], x, y, z),
                grad3d(perm[p10], x - 1.0, y, z),
                fx,
            ),
            lerp(
                grad3d(perm[p01], x, y - 1.0, z),
                grad3d(perm[p11], x - 1.0, y - 1.0, z),
                fx,
            ),
            fy,
        ),
        lerp(
            lerp(
                grad3d(perm[p00 + 1], x, y, z - 1.0),
                grad3d(perm[p10 + 1], x - 1.0, y, z - 1.0),
                fx,
            ),
            lerp(
                grad3d(perm[p01 + 1], x, y - 1.0, z - 1.0),
                grad3d(perm[p11 + 1], x - 1.0, y - 1.0, z - 1.0),
                fx,
            ),
            fy,
        ),
        fz,
    )
}

pub fn perlin2d(opts: &PerlinOpts) -> f64 {
    let mut effect = 1f64;
    let mut k = 1f64;
    let mut sum = opts.base;

    for _ in 0..opts.octaves {
        effect *= opts.fallout;

        sum += effect
            * (1.0
                + noise2d(
                    k * opts.x as f64,
                    k * opts.y as f64,
                    opts.permutations,
                ))
            / 2.0;

        k *= opts.lacunarity
    }

    sum
}

pub fn perlin3d(opts: &PerlinOpts) -> f64 {
    let mut effect = 1.0;
    let mut k = 1.0;
    let mut sum = opts.base;

    for _ in 0..opts.octaves {
        effect *= opts.fallout;

        sum += effect
            * (1.0
                + noise3d(
                    k * opts.x as f64,
                    k * opts.y as f64,
                    k * opts.z as f64,
                    opts.permutations,
                ))
            / 2.0;

        k *= opts.lacunarity
    }

    sum
}

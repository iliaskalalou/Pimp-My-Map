use raylib::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub coords: Vector3,
    pub value: f64,
}

impl Voxel {
    pub fn new(coords: Vector3, value: f64) -> Self {
        Self { coords, value }
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Self::new(Vector3::zero(), 0.0)
    }
}

pub struct VoxelMap {
    pub voxels: Vec<Voxel>,
    pub dims: Vector3,
    pub res: Vector3,
}

impl VoxelMap {
    pub fn new(dims: Vector3, res: Vector3) -> Self {
        let elx = dims.x / res.x;
        let ely = dims.y / res.y;
        let elz = dims.z / res.z;

        let mut res = Self {
            voxels: vec![Voxel::default(); (elx * ely * elz) as usize],
            dims,
            res,
        };

        for k in 0..(elz as usize) {
            for j in 0..(ely as usize) {
                for i in 0..(elx as usize) {
                    res.voxels[k * (ely as usize) + j * (elx as usize) + i]
                        .coords = Vector3::new(i as f32, j as f32, k as f32);
                }
            }
        }

        res
    }

    pub fn render_to_img(&self) -> Image {
        let mut img = Image::gen_image_color(
            self.dims.x as i32,
            self.dims.y as i32,
            Color::WHITE,
        );

        let elx = self.dims.x / self.res.x;
        let ely = self.dims.y / self.res.y;
        let _elz = self.dims.z / self.res.z;

        for j in 0..(ely as usize) {
            for i in 0..(elx as usize) {
                let vox = &self.voxels[j * (elx as usize) + i];
                let col = (vox.value * 255.0) as u8;

                img.draw_rectangle(
                    (vox.coords.x * self.res.x) as i32,
                    (vox.coords.y * self.res.y) as i32,
                    self.res.x as i32,
                    self.res.y as i32,
                    rcolor(col, col, col, 255),
                );
            }
        }

        img
    }
}

impl Default for VoxelMap {
    fn default() -> Self {
        Self::new(Vector3::new(512.0, 512.0, 1.0), Vector3::new(1.0, 1.0, 1.0))
    }
}

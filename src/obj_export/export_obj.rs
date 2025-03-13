use crate::obj_export::mesh::mesh::Mesh;

pub fn create_3d_terrain(
    res_w: usize,
    res_h: usize,
    heightmap: Vec<Vec<f64>>,
    path: &str,
    max_height: f64,
) -> Result<(), String> {
    let new_height_map = resize_mat(&heightmap, res_w, res_h, max_height);
    let mesh = Mesh::terrain(
        heightmap[0].len() as f64,
        heightmap.len() as f64,
        res_w - 1,
        res_h - 1,
        new_height_map,
    );
    mesh.to_obj(path)
}

fn resize_mat(
    mat: &Vec<Vec<f64>>,
    new_size_x: usize,
    new_size_y: usize,
    max_height: f64,
) -> Vec<Vec<f64>> {
    let scaling_factor_x: f64 = new_size_x as f64 / mat.len() as f64;
    let scaling_factor_y: f64 = new_size_y as f64 / mat[0].len() as f64;
    let mut resized_mat = vec![vec![0.0; new_size_y]; new_size_x];
    for i in 0..new_size_x {
        for j in 0..new_size_y {
            let new_i = i as f64 / scaling_factor_x;
            let new_j = j as f64 / scaling_factor_y;
            resized_mat[i][j] =
                mat[new_j as usize][new_i as usize] * max_height;
        }
    }
    resized_mat
}

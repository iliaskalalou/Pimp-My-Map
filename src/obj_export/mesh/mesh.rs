use crate::obj_export::delaunay::delaunay::conquere;
use crate::obj_export::graph::graph::Graph;
use crate::obj_export::vector3::vector3::Vector3;

use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertex: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub indices: Vec<(usize, usize, usize)>,
    pub x_max: f64,
    pub z_max: f64,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertex: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            x_max: 0.0,
            z_max: 0.0,
        }
    }

    pub fn plane(
        width: f64,
        height: f64,
        nb_slice_w: usize,
        nb_slice_h: usize,
    ) -> Self {
        let mut cache_file = match File::create("./.tmp_obj_file") {
            Ok(f) => Some(f),
            Err(e) => {
                println!("Error while creating cache file: {e}");
                None
            }
        };

        let step_x = width / nb_slice_w as f64;
        let step_z = height / nb_slice_h as f64;
        let mut vertex = Vec::new();
        let mut z = 0.0;
        let mut indice = 0;
        let mut points = Vec::new();

        cache_file = write_in_cache(cache_file, "# Vertices and Textures\n");

        for _ in 0..=nb_slice_h {
            let mut x = 0.0;
            for _ in 0..=nb_slice_w {
                vertex.push(Vector3::from(x, 0.0, z, indice));
                points.push(indice);
                cache_file = write_in_cache(
                    cache_file,
                    &format!("v {} {} {}\n", x, 0.0, z),
                );
                cache_file =
                    write_in_cache(cache_file, &format!("vt {} {}\n", z, x));
                indice += 1;
                x += step_x;
            }
            z += step_z;
        }
        vertex[0].y = 100.0;
        cache_file = write_in_cache(cache_file, "# triangles and normals\n");
        let mut graph = Graph::new(points.len(), vertex.clone());
        conquere(&points, nb_slice_w + 1, nb_slice_h + 1, &mut graph);
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        for i in 0..graph.order {
            for j in 0..graph.adjlists[i].len() {
                for k in j + 1..graph.adjlists[i].len() {
                    let elt_j = graph.adjlists[i][j];
                    let elt_k = graph.adjlists[i][k];

                    if graph.adjlists[elt_j].contains(&elt_k) {
                        indices.push((
                            i,
                            graph.adjlists[i][j],
                            graph.adjlists[i][k],
                        ));
                        let s1 = vertex[i].clone();
                        let s2 = &vertex[graph.adjlists[i][j]];
                        let mut vec_normal = s1.vec_product(s2);
                        vec_normal.normalize();
                        cache_file = write_in_cache(
                            cache_file,
                            &format!(
                                "vn {} {} {}\n",
                                vec_normal.x, vec_normal.y, vec_normal.z
                            ),
                        );
                        normals.push(if vec_normal.is_zero() {
                            Vector3::from(0.0, 1.0, 0.0, 0)
                        } else {
                            vec_normal
                        });
                        graph.adjlists[elt_j].retain(|x| *x != i);
                        graph.adjlists[elt_k].retain(|x| *x != i);
                        cache_file = write_in_cache(
                            cache_file,
                            &format!(
                                "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                                i + 1,
                                i + 1,
                                normals.len(),
                                graph.adjlists[i][j] + 1,
                                graph.adjlists[i][j] + 1,
                                normals.len(),
                                graph.adjlists[i][k] + 1,
                                graph.adjlists[i][k] + 1,
                                normals.len(),
                            ),
                        );
                    }
                }
            }
        }

        let mesh = Mesh {
            vertex,
            normals,
            indices,
            x_max: width,
            z_max: height,
        };
        mesh
    }

    pub fn terrain(
        width: f64,
        height: f64,
        nb_slice_w: usize,
        nb_slice_h: usize,
        heightmap: Vec<Vec<f64>>,
    ) -> Self {
        let mut cache_file = match File::create("./.tmp_obj_file") {
            Ok(f) => Some(f),
            Err(e) => {
                println!("Error while creating cache file: {e}");
                None
            }
        };

        let step_x = width / nb_slice_w as f64;
        let step_z = height / nb_slice_h as f64;
        let mut vertex = Vec::new();
        let mut z = 0.0;
        let mut indice = 0;
        let mut points = Vec::new();

        cache_file = write_in_cache(cache_file, "# Vertices and Textures\n");

        for i in 0..=nb_slice_h {
            let mut x = 0.0;
            for j in 0..=nb_slice_w {
                vertex.push(Vector3::from(x, heightmap[i][j], z, indice));
                points.push(indice);
                cache_file = write_in_cache(
                    cache_file,
                    &format!("v {} {} {}\n", x, heightmap[j][i], z),
                );
                cache_file = write_in_cache(
                    cache_file,
                    &format!("vt {} {}\n", 1.0 - x / width, z / height),
                );
                indice += 1;
                x += step_x;
            }
            z += step_z;
        }

        let mut graph = Graph::new(points.len(), vertex.clone());
        conquere(&points, nb_slice_w + 1, nb_slice_h + 1, &mut graph);
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        cache_file = write_in_cache(cache_file, "# triangles and normals\n");
        for i in 0..graph.order {
            for j in 0..graph.adjlists[i].len() {
                for k in j + 1..graph.adjlists[i].len() {
                    let elt_j = graph.adjlists[i][j];
                    let elt_k = graph.adjlists[i][k];

                    if graph.adjlists[elt_j].contains(&elt_k) {
                        indices.push((
                            i,
                            graph.adjlists[i][j],
                            graph.adjlists[i][k],
                        ));
                        let s1 = vertex[i].clone();
                        let s2 = &vertex[graph.adjlists[i][j]];
                        let mut vec_normal = s1.vec_product(s2);
                        vec_normal.normalize();
                        cache_file = write_in_cache(
                            cache_file,
                            &format!(
                                "vn {} {} {}\n",
                                vec_normal.x, vec_normal.y, vec_normal.z
                            ),
                        );
                        normals.push(if vec_normal.is_zero() {
                            Vector3::from(0.0, 1.0, 0.0, 0)
                        } else {
                            vec_normal
                        });
                        graph.adjlists[elt_j].retain(|x| *x != i);
                        graph.adjlists[elt_k].retain(|x| *x != i);
                        cache_file = write_in_cache(
                            cache_file,
                            &format!(
                                "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                                i + 1,
                                i + 1,
                                normals.len(),
                                graph.adjlists[i][j] + 1,
                                graph.adjlists[i][j] + 1,
                                normals.len(),
                                graph.adjlists[i][k] + 1,
                                graph.adjlists[i][k] + 1,
                                normals.len(),
                            ),
                        );
                    }
                }
            }
        }

        let mesh = Mesh {
            vertex,
            normals,
            indices,
            x_max: width,
            z_max: height,
        };
        mesh
    }

    pub fn to_obj(&self, path: &str) -> Result<(), String> {
        if fs::metadata("./.tmp_obj_file").is_ok() {
            match fs::rename("./.tmp_obj_file", path) {
                Ok(_) => return Ok(()),
                Err(_) => (),
            };
        }

        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("Error on file creation: {e}"));
            }
        };

        match file.write_all(b"# Vertices\n") {
            Err(e) => {
                return Err(format!("Error while writing in file: {e}"));
            }
            _ => (),
        }

        for v in &self.vertex {
            let line = format!("v {} {} {}\n", v.x, v.y, v.z);
            match file.write_all(line.as_bytes()) {
                Err(e) => {
                    return Err(format!("Error while writing in file: {e}"));
                }
                _ => (),
            }
        }

        match file.write_all(b"# Normals\n") {
            Err(e) => {
                return Err(format!("Error while writing in file: {e}"));
            }
            _ => (),
        }

        for vn in &self.normals {
            let line = format!("vn {} {} {}\n", vn.x, vn.y, vn.z);
            match file.write_all(line.as_bytes()) {
                Err(e) => {
                    return Err(format!("Error while writing in file: {e}"));
                }
                _ => (),
            }
        }

        match file.write_all(b"# Textures\n") {
            Err(e) => {
                return Err(format!("Error while writing in file: {e}"));
            }
            _ => (),
        }

        for vt in &self.vertex {
            let line = format!("vt {} {}\n", vt.x, vt.z);
            match file.write_all(line.as_bytes()) {
                Err(e) => {
                    return Err(format!("Error while writing in file: {e}"));
                }
                _ => (),
            }
        }

        match file.write_all(b"# Triangles\n") {
            Err(e) => {
                return Err(format!("Error while writing in file: {e}"));
            }
            _ => (),
        }

        let mut j = 1;
        for i in &self.indices {
            let line = format!(
                "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                i.0 + 1,
                j,
                j,
                i.1 + 1,
                j,
                j,
                i.2 + 1,
                j,
                j
            );
            j += 1;
            match file.write_all(line.as_bytes()) {
                Err(e) => {
                    return Err(format!("Error while writing in file: {e}"));
                }
                _ => (),
            }
        }

        Ok(())
    }
}

fn write_in_cache(cache_file: Option<File>, msg: &str) -> Option<File> {
    match cache_file {
        Some(mut f) => match f.write_all(msg.as_bytes()) {
            Ok(_) => Some(f),
            Err(e) => {
                println!("Error while writing in the cache: {e}");
                None
            }
        },
        None => None,
    }
}

use std::fs::File;
use std::io::prelude::*;
use crate::Mesh;
pub fn export_obj(mesh: &Mesh, filename: &String) {
    let mut file: File = File::create(filename).unwrap();
    file.write_all(b"# Vertices\n").expect("Error while writing vertexes");
    for i in &mesh.vertex {
        file.write(
                format!("v {} {} {}\n", i.x, i.y, i.z).as_bytes())
            .expect("Error while writing vertexes");
    }

    file.write_all(b"\n# Normals\n").expect("Error while writing normals");

    for i in &mesh.normals {
        file.write(
                format!("vn {} {} {}\n", i.x, i.y, i.z).as_bytes())
            .expect("Error while writing normals");
    }


    file.write_all(b"\n# Triangles\n").expect("Error while writing triangles");

    for i in &mesh.indices {
        file.write(format!("f {}/{}/{} {}/{}/{} {}/{}/{}\n", 
                i.0+1, 1, 1,
                i.1+1, 1, 1,
                i.2+1, 1, 1).as_bytes()).unwrap();
    }
}

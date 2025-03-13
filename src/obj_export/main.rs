use std::{time::Instant, usize};

use line::line::Line;
use mesh::mesh::Mesh;
use vector3::vector3::Vector3;


mod vector3;
mod mesh;
mod line;
mod delaunay;
mod export;
mod graph;


fn main() {
   let start = Instant::now();
   let mesh = Mesh::plane(2000.0, 2000.0, 10, 10);
   println!("CREATION OF MESH TIME: {:?}", start.elapsed());
 //   dbg!(&mesh);
       
   let start_export = Instant::now();
   mesh.to_obj("./test.obj").unwrap();
   println!("EXPORT IN OBJ TIME: {:?}", start_export.elapsed());
   println!("TOTAL DURATION: {:?}", start.elapsed());
}



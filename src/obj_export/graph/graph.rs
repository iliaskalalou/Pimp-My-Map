use crate::obj_export::vector3::vector3::Vector3;
#[derive(Debug, Clone)]
pub struct Graph {
    pub vertex: Vec<Vector3>,
    pub order: usize,
    pub adjlists: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(order: usize, vertex: Vec<Vector3>) -> Self {
        Self {
            vertex,
            order,
            adjlists: vec![Vec::new(); order],
        }
    }

    pub fn add_edges(&mut self, x: usize, y: usize) -> bool {
        if !self.adjlists[x].contains(&y) && x != y {
            self.adjlists[x].push(y);
            self.adjlists[y].push(x);
            return true;
        }
        false
    }

    pub fn remove_edges(&mut self, x: usize, y: usize) {
        self.adjlists[x].retain(|n| *n != y);
        self.adjlists[y].retain(|n| *n != x);
    }
}

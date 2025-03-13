use raylib::prelude::Texture2D;

pub struct Tile
{
    pub img: Texture2D,
    pub edges: [String; 4],
    pub up: Vec<usize>,
    pub down: Vec<usize>,
    pub right: Vec<usize>,
    pub left: Vec<usize>,
    pub index: usize,
}

fn reverse_string(s: &String) -> String
{
    s.chars().rev().collect()
}

fn compare_string(s1: &String, s2: &String) -> bool
{
    *s1 == reverse_string(s2)
}

impl Tile
{
    pub fn new(img: Texture2D, edges: &[String; 4], i: usize) -> Self
    {
        Self {
            img,
            edges: edges.clone(),
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
            index: i,
        }
    }

    pub fn analyze(&mut self, edges: &Vec<[String; 4]>)
    {
        for (index, edge) in edges.iter().enumerate() {
            for i in 0..4 {
                if compare_string(&edge[i], &self.edges[(i + 2) % 4]) {
                    match i {
                        0 => self.down.push(index),
                        1 => self.left.push(index),
                        2 => self.up.push(index),
                        3 => self.right.push(index),
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn _rotate(&mut self, _num: u32)
    {
        unimplemented!();
    }
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub indice: usize
}

impl Vector3 {
    pub fn new() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0, 
            z: 0.0,
            indice: 0
        }
    }

    pub fn from(x: f64, y: f64, z: f64, indice: usize) -> Self {
        Self {
            x,
            y,
            z,
            indice
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn vec_product(&self, other: &Self) -> Self {
        Self::from(
            self.y * other.z - self.z * other.y, 
            self.z * other.x - self.x * other.z, 
            self.x * other.y - self.y * other.x , 
            0)
    }

    pub fn normalize(&mut self){
        if self.x != 0.0 {
            self.x = self.x / self.x;
        }

        if self.y != 0.0 {
            self.y = self.y / self.y;
        }

        if self.z != 0.0 {
            self.z = self.z / self.z;
        }
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3::from(self.x / rhs, self.y / rhs, self.z / rhs, 0)
    }
}

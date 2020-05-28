use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn from_nb(v: f32) -> Vector {
        Vector { x: v, y: v, z: v }
    }

    pub fn zero() -> Vector {
        Vector::from_nb(0.0)
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Vector {
        let len = self.length();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn inverse(&self) -> Vector {
        Vector {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

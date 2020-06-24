use super::vector::Point;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub sphere: Sphere,
}
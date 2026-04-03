use crate::Color;
use crate::Vector3;

pub struct Light {
    pub position: Vector3,
    pub falloff: f32,
    pub color: Color,
}

impl Light {
    pub fn new(position: Vector3, falloff: f32, color: Color) -> Self {
        Light {
            position,
            falloff,
            color,
        }
    }

    pub fn enabled(&self, target: Vector3) -> bool {
        f32::sqrt(
            f32::powf(target.x - self.position.x, 2.0)
                + f32::powf(target.y - self.position.y, 2.0)
                + f32::powf(target.z - self.position.z, 2.0),
        ) <= self.falloff
    }
}

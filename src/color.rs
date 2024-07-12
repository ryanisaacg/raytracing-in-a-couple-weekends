use std::fmt::Display;

use crate::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Color(pub Vector3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ir = (self.0.x * 255.999) as u32;
        let ig = (self.0.y * 255.999) as u32;
        let ib = (self.0.z * 255.999) as u32;
        write!(f, "{ir} {ig} {ib}")
    }
}

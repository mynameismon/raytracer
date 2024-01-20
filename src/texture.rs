use crate::vec3::Vec3;

pub trait Texture {
    fn value (&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub type Solid = Vec3;

impl Solid {
    pub fn from_colour (x: f32, y: f32, z: f32) -> Self {
	Vec3::from_point(x, y, z)
    }
}

impl Texture for Solid {
    fn value (&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
	self.clone()
    }
}



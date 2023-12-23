use core::ops;
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl ops::Add for Vec3 {
    type Output = Self;
    
    fn add (self, rhs: Self) -> Self{
	Self {
	    x: self.x + rhs.x,
	    y: self.y + rhs.y,
	    z: self.z + rhs.z
	}
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign (&mut self, rhs: Self) {
	*self = Self {
	    x: self.x + rhs.x,
	    y: self.y + rhs.y,
	    z: self.z + rhs.z
	}
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg (self) -> Self {
	Self {
	    x: -self.x,
	    y: -self.y,
	    z: -self.z
	}
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub (self, rhs: Self) -> Self {
	Self {
	    x: self.x - rhs.x,
	    y: self.y - rhs.y,
	    z: self.z - rhs.z
	}
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign (&mut self, rhs: Self) {
	*self = Self {
	    x: self.x - rhs.x,
	    y: self.y - rhs.y,
	    z: self.z - rhs.z
	}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul (self, t: f32) -> Vec3 {
	Vec3 {
	    x: self.x * t,
	    y: self.y * t,
	    z: self.z * t
	}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul (self, rhs: Vec3) -> Vec3 {
	rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign (&mut self, t: f32) {
	*self = Self {
	    x: self.x * t,
	    y: self.y * t,
	    z: self.z * t
	}
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div (self, t: f32) -> Vec3{
	Vec3 {
	    x: self.x / t,
	    y: self.y / t,
	    z: self.z / t
	}
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign (&mut self, t: f32) {
	*self = Self {
	    x: self.x / t,
	    y: self.y / t,
	    z: self.z / t
	}
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{} {} {}",
	       (256.0 * self.x) as u32,
	       (256.0 * self.y) as u32,
	       (256.0 * self.z) as u32)
    }
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new () -> Vec3 {
	Vec3 {
	    x: 0.0,
	    y: 0.0,
	    z: 0.0
	}
    }

    pub fn from_point (x: f32, y: f32, z: f32) -> Vec3 {
	Vec3 {
	    x,
	    y,
	    z
	}
    }
    
    pub fn length_sq (self) -> f32 {
	self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn length (self) -> f32 {
	self.length_sq().sqrt()
    }

    pub fn unit (self) -> Vec3 {
	self / self.length()
    }

    pub fn dot_self (self) -> f32 {
	self.length_sq()
    }

    pub fn dot (self, rhs: Vec3) -> f32 {
	self.x * rhs.x
	    + self.y * rhs.y
	    + self.z * rhs.z
    }

    pub fn cross (self, rhs: Vec3) -> Vec3 {
	Self {
	    x: self.y * rhs.z - self.z * rhs.y,
	    y: self.z * rhs.x - self.x - rhs.z,
	    z: self.x * rhs.y - self.y * rhs.x
	}
    }
}

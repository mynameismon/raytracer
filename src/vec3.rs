use crate::utils::{random, random_range};
use core::fmt;
use core::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;

impl ops::Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f32) {
        *self = Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f32) {
        *self = Self {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self.x.sqrt()) as u32,
            (256.0 * self.y.sqrt()) as u32,
            (256.0 * self.z.sqrt()) as u32
        )
    }
}

#[allow(dead_code)]
impl Vec3 {
    pub const fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub const fn from_point(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn random() -> Vec3 {
        Self {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    #[inline]
    pub fn random_range(r: ops::Range<f32>) -> Vec3 {
        Self {
            x: random_range(r.clone()),
            y: random_range(r.clone()),
            z: random_range(r.clone()),
        }
    }

    #[inline]
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    #[inline]
    pub fn norm(self) -> f32 {
        self.length_sq()
    }

    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn near_zero(self) -> bool {
        let e: f32 = 1e-6;

        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }

    #[inline]
    pub fn reflect_along(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract_along(self, normal: Vec3, rel_ri: f32) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let perpendicular = rel_ri * (self + cos_theta * normal);
        let parallel = -(1.0 - perpendicular.length_sq()).abs().sqrt() * normal;

        perpendicular + parallel
    }
}

#[allow(dead_code)]
#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.dot(v)
}

#[allow(dead_code)]
#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    u.cross(v)
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let v = Vec3::random_range(-1.0..1.0);
        if v.length() > 1.0 {
            return v.unit();
        }
    }
}

#[allow(dead_code)]
#[inline]
pub fn random_unit_hemisphere(normal: Vec3) -> Vec3 {
    let rand_vector = random_unit_vector();

    if normal.dot(rand_vector) > 0.0 {
        rand_vector
    } else {
        -rand_vector
    }
}

#[allow(dead_code)]
#[inline]
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v.reflect_along(normal)
}

#[allow(dead_code)]
#[inline]
pub fn refract(v: Vec3, normal: Vec3, rel_ri: f32) -> Vec3 {
    v.refract_along(normal, rel_ri)
}

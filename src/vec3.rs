use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z, r: x, g: y, b: z}
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
            x: v1.x+v2.x,
            y: v1.y+v2.y,
            z: v1.z+v2.z,
            r: v1.r,
            g: v1.g,
            b: v1.b,
        }
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            x: (v1.y*v2.z-v1.z*v2.y),
            y: -(v1.x*v2.z-v1.z*v2.z),
            z: (v1.x*v2.y-v1.y*v2.x),
            r: v1.r,
            g: v1.g,
            b: v1.b,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.y,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.y,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.y,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
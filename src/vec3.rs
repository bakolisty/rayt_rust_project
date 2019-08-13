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

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x*self.x+self.y*self.y+self.z*self.z)
    }

    pub fn squared_len(&self) -> f32 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    pub fn dot(&self, v2: Vec3) -> f32 {
        self.x*v2.x+self.y*v2.y+self.z*v2.z
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y*v2.z-self.z*v2.y),
            y: -(self.x*v2.z-self.z*v2.z),
            z: (self.x*v2.y-self.y*v2.x),
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub fn make_unit_vec(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x*k,
            y: self.y*k,
            z: self.z*k,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

pub fn unit_vec3(v: Vec3) -> Vec3 {
    v / v.length()
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

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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


impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}
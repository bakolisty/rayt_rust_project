use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{center, radius}
    }
}

impl hitable::Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: hitable::HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - (self.radius*self.radius);
        let discriminant = b*b - (4.0*a*c);
        if discriminant > 0.0 {
            let temp = (-b + f32::sqrt(b*b-a*c)) / a;
            if temp < t_max && temp > t_min {
                
            }
        }
        else {
            (-b - discriminant.sqrt()) / (2.0*a)
        }
        false
    }
}
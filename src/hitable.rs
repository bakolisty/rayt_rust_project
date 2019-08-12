use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord{t, p, normal}
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<sphere::Sphere>,
}

impl HitableList {
    pub fn new(list: Vec<sphere::Sphere>) -> HitableList {
        HitableList{list}
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut maybe_hit: Option<HitRecord> = None;
        for i in self.list.iter() {
            if let Some(hit) = i.hit(&r, t_min, closest_so_far) {
                if hit.t < closest_so_far {
                    maybe_hit = Some(hit);
                    closest_so_far = hit.t;
                }
            }
        }
        maybe_hit
    }
}
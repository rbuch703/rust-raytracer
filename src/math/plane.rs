use crate::math::{Geometry3D, GeometryHitRecord, Vec3};

pub struct Plane {
    point: Vec3,
    normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Plane {
        Plane { point, normal }
    }
}

impl Geometry3D for Plane {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<GeometryHitRecord> {
        //from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let denom = ray_dir.dot(self.normal);
        let num = (&self.point - ray_src).dot(self.normal);
        //println!("test plane with {}/{}", num, denom);
        // denom is zero or num and denom differ in sign --> quotient would be negative
        if denom == 0.0 || num * denom < 0.0 {
            None
        } else {
            Some(GeometryHitRecord {
                distance: num / denom,
                normal: self.normal,
            })
        }
    }
}

use crate::{math::Vec3, scene_objects::{HitRecord, Material, Object3D}};


pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Material,
}


impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Object3D for Plane {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord<'_>> {
        //from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let denom = ray_dir.dot(self.normal);
        let num = (&self.point - ray_src).dot(self.normal);
        //println!("test plane with {}/{}", num, denom);
        // denom is zero or num and denom differ in sign --> quotient would be negative
        if denom == 0.0 || num * denom < 0.0 {
            None
        } else {
            Some(HitRecord {
                distance: num / denom,
                object: self,
                normal: self.normal,
            })
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}


//mod math3d;
use super::math3d::Vec3;


pub struct Material {
    pub color: Vec3,
    pub reflectance: f64,
}

impl Material {
    pub fn new(color: Vec3, reflectance: f64) -> Material {
        Material{color, reflectance}
    }
    pub fn rand(rng: &mut rand::Rng) -> Material {
        Material{
            color: Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()),
            reflectance: rng.next_f64()
        }
    }
}

pub trait Object3D {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord>;
    fn normal_at(&self, pt: Vec3) -> Vec3;
    fn get_material(&self) -> &Material;
}

impl Object3D {
    fn get_color(&self) -> Vec3 {
        self.get_material().color
    }
}

pub struct HitRecord<'a> {
    pub distance: f64,
    pub object: &'a Object3D,
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object3D for Sphere {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord> {
        // from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = ray_src - &self.center;
        //let fac = -Vec3::dot(ray_dir, &oc);
        //let dir_dot_oc = Vec3::dot(ray_dir, &oc);
        let t1 = Vec3::dot(ray_dir, &oc);
        let radicant = t1 * t1 - oc.squared_length() + self.radius * self.radius;

        if radicant < 0.0 {
            return None;
        } else {
            let v1 = -t1;
            let v2 = radicant.sqrt();

            if v1+v2 < 0.0 { // all intersection points lie behind ray_src
                return None;
            }

            return Some(HitRecord {
                distance: match  v1 - v2 >= 0.0 {
                  true => v1 - v2,
                  false =>v1 + v2
                },
                object: self,
            });
        }
    }

    fn normal_at(&self, pt: Vec3) -> Vec3 {
        (&pt - &self.center).normalized()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
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
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord> {
        //from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let denom = Vec3::dot(ray_dir, &self.normal);
        let num = (&self.point - ray_src ).dot(&self.normal);
        //println!("test plane with {}/{}", num, denom);
        // denom is zero or num and denom differ in sign --> quotient would be negative
        if denom == 0.0 || num * denom < 0.0 {
            return None;
        } else {
            return Some(HitRecord {
                distance: num / denom,
                object: self,
            });
        }
    }

    fn normal_at(&self, _pt: Vec3) -> Vec3 {
        self.normal
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

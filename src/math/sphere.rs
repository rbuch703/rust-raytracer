use crate::math::{Bounded3D, BoundingBox, Geometry3D, GeometryHitRecord, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Geometry3D for Sphere {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<GeometryHitRecord> {
        // from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = ray_src - &self.center;
        //let fac = -Vec3::dot(ray_dir, &oc);
        //let dir_dot_oc = Vec3::dot(ray_dir, &oc);
        let t1 = ray_dir.dot(oc);
        let radicant = t1 * t1 - oc.squared_length() + self.radius * self.radius;

        if radicant < 0.0 {
            None
        } else {
            let v1 = -t1;
            let v2 = radicant.sqrt();

            if v1 + v2 < 0.0 {
                // all intersection points lie behind ray_src
                None
            } else {
                let distance = if v1 - v2 >= 0.0 { v1 - v2 } else { v1 + v2 };
                let hit_point = ray_src + ray_dir * distance;
                Some(GeometryHitRecord {
                    distance,
                    normal: (hit_point - self.center).normalized(),
                })
            }
        }
    }
}

impl Bounded3D for Sphere {
    fn bounds(&self) -> BoundingBox {
        (self.center - Vec3::new(1.0, 1.0, 1.0) * self.radius).bounds()
            | (self.center + Vec3::new(1.0, 1.0, 1.0) * self.radius).bounds()
    }
}

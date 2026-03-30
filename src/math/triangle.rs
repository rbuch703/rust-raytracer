use crate::math::{Bounded3D, BoundingBox, Geometry3D, GeometryHitRecord, Vec3};

pub struct Triangle<T> {
    pub v1: T,
    pub v2: T,
    pub v3: T,
}

impl<T> Triangle<T> {
    pub fn new(v1: T, v2: T, v3: T) -> Self {
        Triangle { v1, v2, v3 }
    }
}

impl Geometry3D for Triangle<Vec3> {
    fn hit(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<super::GeometryHitRecord> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;

        let ray_cross_e2 = ray_direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = *ray_origin - self.v1;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray_direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * e2.dot(s_cross_e1);

        if t > f64::EPSILON {
            // ray intersection
            Some(GeometryHitRecord {
                distance: t,
                normal: e1.cross(e2).normalized(),
            })
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
}

impl Bounded3D for Triangle<Vec3> {
    fn bounds(&self) -> BoundingBox {
        self.v1.bounds() | self.v2.bounds() | self.v3.bounds()
    }
}

use crate::math::{BoundingBox, Geometry3D, GeometryHitRecord, Vec3};

pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub n1: Vec3,
    pub n2: Vec3,
    pub n3: Vec3,
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        let normal = (v2 - v1).cross(v3 - v1).normalized();
        Triangle { v1, v2, v3, n1: normal, n2: normal, n3: normal }
    }
}

impl Geometry3D for Triangle {
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
            let normal = (self.n1 * (1.0 - u - v) + self.n2 * u + self.n3 * v).normalized();
            Some(GeometryHitRecord {
                distance: t,
                normal,
            })
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }

    fn bounds(&self) -> Option<BoundingBox> {
        BoundingBox::from_vertices(&[self.v1, self.v2, self.v3])
    }
}

use crate::{
    math3d::Vec3,
    scene_objects::{HitRecord, Material, Object3D},
};

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

impl Triangle<Vec3> {
    // Taken from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn intersects(&self, ray_origin: Vec3, ray_direction: Vec3) -> Option<Vec3> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;

        let ray_cross_e2 = ray_direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray_origin - self.v1;
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
            Some(ray_origin + ray_direction * t)
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
}

pub fn parse_obj(filename: &str) -> std::io::Result<Vec<Triangle<Vec3>>> {
    let mut vertices = Vec::new();
    let mut faces = Vec::<(usize, usize, usize)>::new();

    for l in std::fs::read_to_string(filename)?.lines() {
        let line = l.trim();
        if line.is_empty() {
            // skip emtpy lines
            continue;
        }

        if line.starts_with('#') {
            // skip comment lines
            continue;
        }

        let mut parts = line.split(' ');
        let mut next = || {
            parts
                .next()
                .ok_or(std::io::Error::from(std::io::ErrorKind::Other))
        };
        let indicator = next()?;
        match indicator {
            "f" => {
                faces.push((
                    next()?.parse().expect("int"),
                    next()?.parse().expect("int"),
                    next()?.parse().expect("int"),
                ));
            }
            "v" => {
                vertices.push(Vec3::new(
                    next()?.parse().expect("f64"),
                    next()?.parse().expect("f64"),
                    next()?.parse().expect("f64"),
                ));
            }
            _ => panic!("Unexpected indicator {}", indicator),
        }
        if let Ok(el) = next() {
            panic!("Found trailing element {el}");
        }
    }

    // Obj face indices are one-based, so shift to zero-based array indices
    Ok(faces
        .into_iter()
        .map(|(i1, i2, i3)| Triangle::new(vertices[i1 - 1], vertices[i2 - 1], vertices[i3 - 1]))
        .collect())
}

pub struct TriangleMesh {
    triangles: Vec<Triangle<Vec3>>,
    material: Material,
}

fn map_vertices<T, U, F>(src: &[Triangle<T>], trans: F) -> Vec<Triangle<U>>
where
    F: Fn(&T) -> U,
{
    src.iter()
        .map(|Triangle { v1, v2, v3 }| Triangle::new(trans(v1), trans(v2), trans(v3)))
        .collect()
}

impl TriangleMesh {
    pub fn from_obj_file(file_name: &str, material: Material) -> std::io::Result<TriangleMesh> {
        let triangles = parse_obj(file_name)?;

        let triangles = map_vertices(&triangles, |v| {
            Vec3::new(
                v.x * 10000.0,
                v.y * -10000.0 + 500.0,
                -v.z * 10000.0 + 1500.0,
            )
        });

        Ok(TriangleMesh {
            triangles,
            material,
        })
    }
}

impl Object3D for TriangleMesh {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        for triangle in &self.triangles {
            if let Some(hit_pos) = triangle.intersects(*ray_src, *ray_dir) {
                let distance = (ray_src - &hit_pos).len();
                let this_is_best = match &best_hit {
                    Some(hit) => hit.distance > distance,
                    None => true,
                };
                if this_is_best {
                    let e1 = triangle.v3 - triangle.v1;
                    let e2 = triangle.v2 - triangle.v1;
                    best_hit = Some(
                        HitRecord { distance, object: self, normal: /*-ray_dir*/-e1.cross(e2).normalized() },
                    );
                }
            }
        }

        best_hit
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

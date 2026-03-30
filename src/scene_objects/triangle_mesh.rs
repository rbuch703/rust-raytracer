use crate::{
    math::{Bounded3D, BoundingBox, Geometry3D, GeometryHitRecord, Triangle, Vec3},
    scene_objects::mesh::KDTreeNode,
    utils::parse_obj,
};

pub struct TriangleMesh {
    geometry: KDTreeNode,
    bounds: BoundingBox,
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
    pub fn from_obj_file(file_name: &str) -> std::io::Result<TriangleMesh> {
        let triangles = parse_obj(file_name)?;

        let triangles = map_vertices(&triangles, |v| {
            Vec3::new(
                v.x * 10000.0,
                v.y * -10000.0 + 500.0,
                -v.z * 10000.0 + 1500.0,
            )
        });

        // The mesh is only valid if it has at least one triangle (Otherwise, no bounds can be established)
        let mut bounds = triangles
            .first()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?
            .v1
            .bounds();
        for t in &triangles {
            bounds |= t.v1;
            bounds |= t.v2;
            bounds |= t.v3;
        }
        Ok(TriangleMesh {
            geometry: KDTreeNode::new(triangles),
            bounds,
        })
    }
}

impl Geometry3D for TriangleMesh {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<GeometryHitRecord> {
        let mut best_hit = None;
        self.geometry.hit_recursive(ray_src, ray_dir, &mut best_hit);

        best_hit.map(|hit| GeometryHitRecord {
            distance: hit.distance,
            normal: hit.normal,
        })
    }
}

impl Bounded3D for TriangleMesh {
    fn bounds(&self) -> BoundingBox {
        self.bounds
    }
}

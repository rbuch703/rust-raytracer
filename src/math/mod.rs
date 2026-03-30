#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

// Any object that has 3D axis-aligned bounds
pub trait Bounded3D {
    fn bounds(&self) -> BoundingBox;
}

#[derive(Clone, Copy)]
pub struct GeometryHitRecord {
    pub distance: f64,
    pub normal: Vec3,
}

impl GeometryHitRecord {
    pub fn min(&self, other: GeometryHitRecord) -> Self {
        if self.distance < other.distance {
            *self
        } else {
            other
        }
    }
}

pub trait Geometry3D {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<GeometryHitRecord>;
}

mod vec3;
pub use vec3::Vec3;

mod bounding_box;
pub use bounding_box::BoundingBox;

mod plane;
pub use plane::Plane;

mod range;
pub use range::Range;

mod sphere;
pub use sphere::Sphere;

mod triangle;
pub use triangle::Triangle;

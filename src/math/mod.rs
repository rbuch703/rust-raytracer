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

mod vec3;
pub use vec3::Vec3;

mod bounding_box;
pub use bounding_box::BoundingBox;

mod range;
pub use range::Range;


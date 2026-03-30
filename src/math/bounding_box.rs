use std::ops::{BitOr, BitOrAssign};

use crate::math::{Axis, Geometry3D, Range, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub x: Range,
    pub y: Range,
    pub z: Range,
}

impl BitOrAssign<&BoundingBox> for BoundingBox {
    fn bitor_assign(&mut self, rhs: &BoundingBox) {
        self.x |= rhs.x;
        self.y |= rhs.y;
        self.z |= rhs.z;
    }
}

impl BitOrAssign<Vec3> for BoundingBox {
    fn bitor_assign(&mut self, rhs: Vec3) {
        self.x |= rhs.x;
        self.y |= rhs.y;
        self.z |= rhs.z;
    }
}

/*
impl <T> TryFrom<&[T]> for BoundingBox where T: Bounded3D {
    type Error;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        todo!()
    }
}*/

impl BoundingBox {
    pub fn from_items<T>(items: &[impl Geometry3D]) -> Option<BoundingBox> {
        let mut res = None;

        for item in items {
            if let Some(bounds) = item.bounds() {
                if let Some(mut res) = res {
                    res |= &bounds;
                } else {
                    res = Some(bounds);
                }
            }
        }

        res
    }

    pub fn from_vertices(vertices: &[Vec3]) -> Option<BoundingBox> {
        let (mut x, mut y, mut z) = if let Some(first) = vertices.first() {
            (
                Range::new(first.x),
                Range::new(first.y),
                Range::new(first.z),
            )
        } else {
            return None;
        };

        for v in &vertices[1..] {
            x |= v.x;
            y |= v.y;
            z |= v.z;
        }

        Some(BoundingBox { x, y, z })
    }

    pub fn get(&self, axis: Axis) -> Range {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn _min(&self) -> Vec3 {
        Vec3::new(self.x.min, self.y.min, self.z.min)
    }

    pub fn _max(&self) -> Vec3 {
        Vec3::new(self.x.max, self.y.max, self.z.max)
    }
}

impl BitOr<BoundingBox> for BoundingBox {
    type Output = BoundingBox;

    fn bitor(self, rhs: BoundingBox) -> Self::Output {
        BoundingBox {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }
}

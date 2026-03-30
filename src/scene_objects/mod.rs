mod mesh;
use crate::math::{BoundingBox, Geometry3D, Vec3};
pub use mesh::KDTreeNode;

pub struct Material {
    pub color: Vec3,
    pub reflectance: f64,
    pub specular_strength: f64,
    pub specular_exponent: f64,
}

impl Material {
    pub fn new(
        color: Vec3,
        reflectance: f64,
        specular_strength: f64,
        specular_exponent: f64,
    ) -> Material {
        Material {
            color,
            reflectance,
            specular_strength,
            specular_exponent,
        }
    }

    pub fn new_diffuse(color: Vec3) -> Material {
        Material {
            color,
            reflectance: 0.0,
            specular_strength: 0.0,
            specular_exponent: 0.0,
        }
    }

    pub fn _rand(rng: &mut dyn rand::RngCore) -> Material {
        use crate::rand::Rng;
        Material {
            color: Vec3::new(rng.r#gen::<f64>(), rng.r#gen::<f64>(), rng.r#gen::<f64>()),
            reflectance: rng.r#gen::<f64>(),
            specular_strength: rng.r#gen::<f64>(),
            specular_exponent: rng.r#gen::<f64>() * 10.0,
        }
    }
}

//pub type SceneObject = dyn Object3D + Sync + Send;
pub trait SceneObject {
    fn hit(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<ObjectHitRecord<'_>>;
}

pub struct SimpleSceneObject {
    geometry: Box<dyn Geometry3D + Sync + Send>,
    material: Material,
}

impl SimpleSceneObject {
    pub fn new(
        geometry: impl Geometry3D + Sync + Send + 'static,
        material: Material,
    ) -> SimpleSceneObject {
        SimpleSceneObject {
            geometry: Box::new(geometry),
            material,
        }
    }

    pub fn _from_random_material(
        geometry: Box<dyn Geometry3D + Sync + Send>,
        rng: &mut dyn rand::RngCore,
    ) -> SimpleSceneObject {
        SimpleSceneObject {
            geometry,
            material: Material::_rand(rng),
        }
    }

    pub fn bounds(&self) -> Option<BoundingBox> {
        self.geometry.bounds()
    }
}

impl SceneObject for SimpleSceneObject {
    fn hit(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<ObjectHitRecord<'_>> {
        self.geometry
            .hit(ray_origin, ray_direction)
            .map(|hit_record| ObjectHitRecord {
                distance: hit_record.distance,
                normal: hit_record.normal,
                material: &self.material,
            })
    }
}

pub struct ObjectHitRecord<'a> {
    pub distance: f64,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> ObjectHitRecord<'a> {
    pub fn min(self, other: ObjectHitRecord<'a>) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }
}

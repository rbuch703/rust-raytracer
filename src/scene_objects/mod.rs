mod mesh;
use crate::math::{Geometry3D, Vec3};

mod triangle_mesh;
pub use triangle_mesh::TriangleMesh;

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

pub struct SceneObject {
    geometry: Box<dyn Geometry3D + Sync + Send>,
    material: Material,
}

impl SceneObject {
    pub fn new(
        geometry: impl Geometry3D + Sync + Send + 'static,
        material: Material,
    ) -> SceneObject {
        SceneObject {
            geometry: Box::new(geometry),
            material,
        }
    }

    pub fn _from_random_material(
        geometry: Box<dyn Geometry3D + Sync + Send>,
        rng: &mut dyn rand::RngCore,
    ) -> SceneObject {
        SceneObject {
            geometry,
            material: Material::_rand(rng),
        }
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn hit(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<ObjectHitRecord<'_>> {
        self.geometry
            .hit(ray_origin, ray_direction)
            .map(|hit_record| ObjectHitRecord {
                distance: hit_record.distance,
                normal: hit_record.normal,
                object: self,
            })
    }
}

pub struct ObjectHitRecord<'a> {
    pub distance: f64,
    pub normal: Vec3,
    pub object: &'a SceneObject,
}

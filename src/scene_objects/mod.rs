mod sphere;
mod plane;
mod mesh;
pub use sphere::Sphere;
pub use plane::Plane;
pub use mesh::TriangleMesh;

use crate::math::Vec3;

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

pub trait Object3D {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord<'_>>;
    fn get_material(&self) -> &Material;
}

pub type SceneObject = dyn Object3D + Sync + Send;

pub struct HitRecord<'a> {
    pub distance: f64,
    pub object: &'a dyn Object3D,
    pub normal: Vec3,
}


extern crate rand;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

use std::fmt;
use std::ops;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // trucate to f32 to show fewer digits
        write!(
            f,
            "({}, {}, {})",
            self.x as f32,
            self.y as f32,
            self.z as f32
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> ops::Neg for &'a Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new( - self.x, - self.y, - self.z)
    }
}


impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<'a, 'b> ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'a Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn normalized(&self) -> Vec3 {
        let l = self.len();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        //copied from the internet, not verified
        return Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        );
    }

    pub fn reflect_at(&self, normal: &Vec3) -> Vec3 {
        //copied from the internet, not verified
        self - &(normal * Vec3::dot(&self, &normal) * 2.0)
    }

    pub fn get_cosine_distributed_random_ray(&self, rng: &mut rand::Rng) -> Vec3 {
        // Step 1:Compute a uniformly distributed point on the unit disk
        let r = f64::sqrt(rng.next_f64());
        let phi = 2.0 * std::f64::consts::PI * rng.next_f64();

        // Step 2: Project point onto unit hemisphere
        let u = r * f64::cos(phi);
        let v = r * f64::sin(phi);
        let n = f64::sqrt(1.0 - r * r);


        let udir = match f64::abs(Vec3::dot(&Vec3::new(0.0, 0.0, 1.0), &self)) >= 0.999999 {
            false => Vec3::new(0.0, 0.0, 1.0),
            //are (nearly) colinear --> cannot build coordinate base
            true => Vec3::new(0.0, 1.0, 0.0),
        };

        let vdir = Vec3::cross(&udir, &self).normalized();
        let udir = Vec3::cross(&vdir, &self).normalized();

        //# Convert to a direction on the hemisphere defined by the normal
        udir * u + vdir * v + self * n
    }
}
    
struct BoundingBox {
    min: Vec3,
    max: Vec3 
}

impl BoundingBox {
    fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox{min, max}
    }

}



#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

use std::fmt;
use std::ops;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // trucate to f32 to show fewer digits
        write!(f, "({}, {}, {})", self.x as f32, self.y as f32, self.z as f32)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs : Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs : Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    // FIXME: why can't rhs be borrowed?
    fn sub(self, rhs : Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<'a, 'b>  ops::Sub<&'a Vec3> for &'b Vec3 {
    type Output = Vec3;

    fn sub(self, rhs : &'a Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs:f64) -> Vec3 {
        Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

impl<'a> ops::Mul<f64> for &'a Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs:f64) -> Vec3 {
        Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}


impl Vec3{
  pub fn new(x:f64, y:f64, z:f64)->Vec3 {
    Vec3{x,y,z}
  }
  
  pub fn len(&self) -> f64{
    (self.x*self.x+
     self.y*self.y+
     self.z*self.z).sqrt()
  }
  
  pub fn squared_length(&self) -> f64 {
    self.x*self.x+
    self.y*self.y+
    self.z*self.z
  }
  
  pub fn dot(&self, rhs: &Vec3) -> f64 {
    self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
  }

  pub fn normalized(&self) -> Vec3
  {
    let l = self.len();
    Vec3{ x: self.x/l, y: self.y/ l, z: self.z/l}
  }
}


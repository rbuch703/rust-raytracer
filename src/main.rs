
extern crate png;
use std::fmt;
use std::ops;

//#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

struct Sphere {
    center: Vec3,
    radius: f64,
    color: Vec3
}

struct HitRecord<'a> {
    distance: f64,
    object: &'a Sphere
}


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
  fn new(x:f64, y:f64, z:f64)->Vec3 {
    Vec3{x,y,z}
  }
  
  fn len(&self) -> f64{
    (self.x*self.x+
     self.y*self.y+
     self.z*self.z).sqrt()
  }
  
  fn squared_length(&self) -> f64 {
    self.x*self.x+
    self.y*self.y+
    self.z*self.z
  }
  
  fn dot(&self, rhs: &Vec3) -> f64 {
    self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
  }

  fn normalized(&self) -> Vec3
  {
    let l = self.len();
    Vec3{ x: self.x/l, y: self.y/ l, z: self.z/l}
  }
}

impl Sphere{
    fn new( center:Vec3, radius:f64, color:Vec3) -> Sphere
    {
        Sphere{center, radius, color}   
    }
    
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord>
    {
        // from https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = ray_src - &self.center;
        //let fac = -Vec3::dot(ray_dir, &oc);
        //let dir_dot_oc = Vec3::dot(ray_dir, &oc);
        let t1 = Vec3::dot(ray_dir, &oc);
        let radicant = t1*t1 - oc.squared_length() + self.radius*self.radius;
        
        if radicant < 0.0 {
            return None
        } else {
            let v1 = -t1;
            let v2 = radicant.sqrt();
            //let d : f64;
            if v1 - v2 >= 0.0 {
                return Some(HitRecord{distance:v1-v2, object:self});
            } else {
                return Some(HitRecord{distance:v1+v2, object:self});
            }
        }
    }

    fn normal_at(&self, pt: Vec3) -> Vec3 {
        (&pt - &self.center).normalized()
    }
}

fn main() {

// For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // To use encoder.set()
    use png::HasParameters;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 511, 511); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    
    let mut image_data: Vec<u8> = Vec::new();
 
    let s = Sphere::new( Vec3::new(50.0, 50.0, 1000.0), 200.0, Vec3::new(1.0, 0.0, 0.5));
    let s2 = Sphere::new( Vec3::new(-50.0, 50.0, 1000.0), 500.0, Vec3::new(0.2, 1.0, 0.1));

    let ray_src = Vec3::new(0.0, 0.0, 0.0);
    let light_dir = Vec3::new(-1.0, -1.0, -1.0).normalized();

    for y in -255i16..256 {
        for x in -255i16..256 {
            let v = Vec3::new(x as f64, y as f64, 255.0).normalized();

            let mut hit_obj = s.hit( &ray_src, &v);
            match hit_obj {
                None => hit_obj= s2.hit(&ray_src, &v),
                Some(_) => {}
            }
            
            if let Some(obj) = hit_obj {
                let p_hit = &ray_src + &v* obj.distance;
                let n = obj.object.normal_at(p_hit);
                let dot = Vec3::dot(&n,&light_dir);
                let diffuse = match dot > 0.0 {
                    true => dot,
                    false => 0.0
                };
                let col = &obj.object.color*diffuse;
                image_data.push( (col.x*255.0) as u8);
                image_data.push( (col.y*255.0) as u8);
                image_data.push( (col.z*255.0) as u8);
                image_data.push(255);            
            } else {
                //let f = Vec3::dot(&v, &dir);
                let v_col = (v + Vec3::new(1.0, 1.0,1.0)) *  0.5;
                image_data.push( (v_col.x*255.0) as u8);
                image_data.push( (v_col.y*255.0) as u8);
                image_data.push( (v_col.z*255.0) as u8);
                image_data.push(255);            

            }
        }
    }

    writer.write_image_data(&image_data).unwrap(); // Save
}

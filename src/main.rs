
extern crate png;

mod math3d;
use math3d::Vec3;

//#[derive(Debug)]

struct Sphere {
    center: Vec3,
    radius: f64,
    color: Vec3
}

struct Plane {
    point: Vec3,
    normal: Vec3,
    color: Vec3
}

trait Object3D {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord>;
    fn normal_at(&self, pt: Vec3) -> Vec3;
    fn get_color(&self) -> &Vec3;
}


struct HitRecord<'a> {
    distance: f64,
    object: &'a Object3D
}

impl Sphere{
    fn new( center:Vec3, radius:f64, color:Vec3) -> Sphere
    {
        Sphere{center, radius, color}   
    }
}

impl Object3D for Sphere {
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

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

}


impl Plane{
    fn new( point:Vec3, normal:Vec3, color:Vec3) -> Plane
    {
        Plane{point, normal, color}   
    }
}

impl Object3D for Plane {
    fn hit(&self, ray_src: &Vec3, ray_dir: &Vec3) -> Option<HitRecord>
    {
        //from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let denom = Vec3::dot(ray_dir, &self.normal);
        let num = (ray_src - &self.point).dot(&self.normal);
        
        // denom is zero or num and denom differ in sign --> quotient would be negative
        if denom == 0.0 || num*denom < 0.0{
            return None;
        } else {
            return Some(HitRecord{distance:num/denom, object:self})
        }
    }

    fn normal_at(&self, _pt: Vec3) -> Vec3 {
       self.normal
    }
    
    fn get_color(&self) -> &Vec3 {
        &self.color
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

    let mut encoder = png::Encoder::new(w, 1023, 767);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    
    let mut image_data: Vec<u8> = Vec::new();

    let mut objects : Vec<Box<Object3D>> = Vec::new();
    objects.push( Box::new(Sphere::new( Vec3::new(-100.0, -80.0, 400.0), 40.0, Vec3::new(0.8, 0.8, 0.8))));
    objects.push( Box::new(Sphere::new( Vec3::new(100.0, -80.0, 400.0), 40.0, Vec3::new(0.8, 0.8, 0.8))));
    objects.push( Box::new(Sphere::new( Vec3::new(0.0, 0.0, 700.0), 350.0, Vec3::new(0.8, 0.8, 0.0))));
    objects.push( Box::new(Sphere::new( Vec3::new(100.0, -80.0, 370.0), 20.0, Vec3::new(0.1, 0.1, 0.1))));
    objects.push( Box::new(Sphere::new( Vec3::new(-100.0, -80.0, 370.0), 20.0, Vec3::new(0.1, 0.1, 0.1))));

    objects.push( Box::new(Plane::new( Vec3::new(0.0,-200.0,0.0), Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.1, 0.5, 0.1))));

    let ray_src = Vec3::new(0.0, 0.0, 0.0);
    let light_dir = Vec3::new(-1.0, -1.0, -1.0).normalized();

    for y in -383i16..384 {
        for x in -511i16..512 {

            let v = Vec3::new(x as f64, y as f64, 512.0).normalized();
            let mut hit_obj : Option<HitRecord> = None;
            
            for obj in objects.iter() {
                if let Some(hit) = obj.hit( &ray_src, &v) {
                    if let Some(old_hit) = hit_obj {
                        if hit.distance < old_hit.distance {
                            hit_obj = Some(hit);
                        } else {
                            hit_obj = Some(old_hit);
                        }
                    } else {
                        hit_obj = Some(hit)
                    
                    }
                }
            }

/*            match hit_obj {
                None => hit_obj= s2.hit(&ray_src, &v),
                Some(_) => {}
            }*/
            
            if let Some(obj) = hit_obj {
                let p_hit = &ray_src + &v* obj.distance;
                let n = obj.object.normal_at(p_hit);
                let dot = Vec3::dot(&n,&light_dir);
                let diffuse = match dot > 0.0 {
                    true => dot,
                    false => 0.0
                };
                let col : &Vec3 = &obj.object.get_color();
                let col = col*diffuse;
//                let col = Vec3::new(1.0, 1.0, 1.0)* obj.distance * 0.001;
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

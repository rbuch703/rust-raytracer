extern crate png;
extern crate rand;
extern crate num_cpus;

mod math3d;
mod scene_objects;
use math3d::Vec3;

type SceneObject = dyn scene_objects::Object3D+Sync+Send;

fn set_normal( pixel: &mut [u8], n:&Vec3) {
    let normalized = (n + Vec3::new(1.0, 1.0, 1.0))* 0.5;
    set_color( pixel, &normalized);
}

fn set_color( pixel: &mut [u8], col:&Vec3) {
    pixel[0] = ( col.x * 255.0) as u8;
    pixel[1] = ( col.y * 255.0) as u8;
    pixel[2] = ( col.z * 255.0) as u8;
    pixel[3] = 255;
}

fn ambient_occlusion(objects: &Vec<Box<SceneObject>>, pos: &Vec3, normal: &Vec3, rng: &mut dyn rand::RngCore, num_samples: u32, distance_cutoff: f64) -> f64 {
    let mut num_hits = 0;
    for _ in 0..num_samples {
        let d = normal.get_cosine_distributed_random_ray(rng);
        if let Some(hit) = trace_ray( &objects, &pos, &d) {
            if hit.distance < distance_cutoff {
                num_hits += 1;
            }
        }
    }

    1.0 - (num_hits as f64) / (num_samples as f64)
}

fn trace_ray<'a>(objects: &'a Vec<Box<SceneObject>>, ray_src: &Vec3, ray_dir: &Vec3) -> Option<scene_objects::HitRecord<'a>>
{
//    println!("tracing ray from {} with {}", ray_src, ray_dir);
    let mut hit_obj: Option<scene_objects::HitRecord> = None;

    for obj in objects.iter() {
        if let Some(hit) = obj.hit(&ray_src, &ray_dir) {
            //println!("\tHit obj {} at {}", hit.object.get_material().color, (ray_src + ray_dir*hit.distance));
            if let Some(ref mut best_hit) = hit_obj {
                if hit.distance < best_hit.distance {
                    *best_hit = hit;
                }
            } else {
                hit_obj = Some(hit)
            }
        }
    }

    hit_obj
}

fn get_color(objects: &Vec<Box<SceneObject>>, ray_src: &Vec3, ray_dir: &Vec3, light_dir: &Vec3, rng: &mut dyn rand::RngCore, recursion_depth: u32 ) -> Vec3 {
    if recursion_depth > 5 {
        return Vec3::new(0.5, 0.5, 0.5);
    }

    let ambient = 0.1;

    let hit_obj = trace_ray(&objects, &ray_src, &ray_dir);

    if let Some(obj) = hit_obj {
        let p_hit = ray_src + ray_dir * obj.distance;
        let n = obj.object.normal_at(p_hit);
        let p_hit = p_hit + n * 1E-7;

        let diffuse = Vec3::dot(&n, &light_dir);
        let diffuse = match diffuse > 0.0 {
            true => diffuse,
            false => 0.0
        };
        
        let material = &obj.object.get_material();
        let light_color = Vec3::new(1.0, 0.7, 0.8);
        let r = ray_dir.reflect_at(&n);
        let specular = clamp(Vec3::dot(&r, &light_dir), 0.0, 1.0).powf(material.specular_exponent) * material.specular_strength;

        let brightness = (diffuse + ambient) * ambient_occlusion(&objects, &p_hit, &n, rng, 100, 200.0);
        let color = material.color * brightness + light_color * specular;
        let reflectance = material.reflectance;
        return match reflectance > 0.0 {
            true => color * (1.0-reflectance) + get_color(objects, &p_hit, &ray_dir.reflect_at(&n), light_dir, rng, recursion_depth+1)* reflectance,
            false => color
        }
    } else
    {
        return Vec3::new(0.0, 0.3, 0.8);
    }
}


fn clamp(v: f64, min: f64, max: f64) -> f64{
    if v < min {
        return min;
    }

    if v > max {
        return max
    }

    v
}

fn create_scene() -> Vec<Box<SceneObject>> {
    let mut objects: Vec<Box<SceneObject>> = Vec::new();

    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(-100.0, -80.0, 400.0),
        40.0,
        scene_objects::Material::new_diffuse(Vec3::new(0.8, 0.8, 0.8)),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(100.0, -80.0, 400.0),
        40.0,
        scene_objects::Material::new_diffuse(Vec3::new(0.8, 0.8, 0.8)),
    )));
/*    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(0.0, 50.0, 700.0),
        350.0,
        scene_objects::Material::new(Vec3::new(0.8, 0.8, 0.0), 0.1, 1.0, 10.0),
    )));*/
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(100.0, -80.0, 370.0),
        20.0,
        scene_objects::Material::new(Vec3::new(0.1, 0.1, 0.1), 0.0, 1.0, 20.0),
    )));
    objects.push(Box::new(scene_objects::Sphere::new(
        Vec3::new(-100.0, -80.0, 370.0),
        20.0,
        scene_objects::Material::new(Vec3::new(0.1, 0.1, 0.1), 0.0, 1.0, 20.0),
    )));
/*
    objects.push(Box::new(scene_objects::Plane::new(
        Vec3::new(0.0, 200.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        scene_objects::Material::new_diffuse(Vec3::new(0.1, 0.5, 0.1)),
    )));*/

    use rand::{SeedableRng};
    let mut rng = rand::rngs::SmallRng::from_entropy();
    use crate::rand::Rng;
    for _i in 0..100 {
        objects.push(Box::new(scene_objects::Sphere::new(
            Vec3::new( (rng.gen::<f64>()-0.5) * 2000.0, (rng.gen::<f64>()-0.5) * 2000.0, rng.gen::<f64>() * 4000.0),
            100.0,
            scene_objects::Material::new( Vec3::new(0.5 + 0.5*rng.gen::<f64>(), 0.5 + 0.5*rng.gen::<f64>(), 0.5+0.5*rng.gen::<f64>()), 0.3, 1.0, 50.0),
        )));
    
    }
    
    objects
}

fn trace_line(row: &mut[u8], row_idx: i16, objects: &Vec<Box<SceneObject>>) {
    println!("Tracing line {}", row_idx);
    let ray_src = Vec3::new(0.0, 0.0, 0.0);
    let light_dir = Vec3::new(-1.0, -1.0, -1.0).normalized();

    use rand::SeedableRng;
    let mut rng = rand::rngs::SmallRng::from_entropy();

    let readable_objects = &*objects;

    for x in 0i16..1023 {

        let v = Vec3::new( (x-511) as f64, (row_idx-383) as f64, 512.0).normalized();
        /*
        if let Some(hit) = trace_ray(&readable_objects, &ray_src, &v) {
            let p_hit = ray_src + v * hit.distance;
            let n = hit.object.normal_at(p_hit);
            let p_hit = p_hit + n * 1E-7;
            set_color( &mut row[(x*4) as usize..], &(Vec3::new(1.0, 1.0, 1.0)* ambient_occlusion(&readable_objects, &p_hit, &n, &mut rng, 1000, 200.0)));
        } else {
            set_color( &mut row[(x*4) as usize..], &Vec3::new(0.0, 0.3, 0.8));
        }*/

        let col = get_color( &objects, &ray_src, &v, &light_dir, &mut rng, 0);
        let col = Vec3::new(
            clamp(col.x, 0.0, 1.0).sqrt(),
            clamp(col.y, 0.0, 1.0).sqrt(),
            clamp(col.z, 0.0, 1.0).sqrt(),
        );
        set_color( &mut row[(x*4) as usize..], &col);

}
 
}

fn take_one<'a>(tasks: &'a std::sync::Arc::<std::sync::Mutex::<&mut std::collections::LinkedList::<(usize, &mut [u8])>>>) -> Option<(usize, &'a mut [u8])>
{
    tasks.lock().unwrap().pop_front()
}

fn main() {

    // For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;

    let objects = std::sync::Arc::new(create_scene());

    let mut image_data: Vec<u8> = vec![0; 767*1023*4];
    let mut tasks : std::collections::LinkedList::<(usize, &mut [u8])> = image_data.chunks_mut(1023*4).enumerate().collect();
    let mut shared_tasks = std::sync::Arc::new(std::sync::Mutex::new(&mut tasks));
    
    let _ = std::thread::scope(|scope| {
        // create as many worker threads as we have CPU cores
        for i in 1..=num_cpus::get()
        {
            let objects_clone = std::sync::Arc::clone(&objects);
            let shared_tasks_clone = std::sync::Arc::clone(&shared_tasks);

            scope.spawn(move||{
                while let Some((idx, row)) = take_one(&shared_tasks_clone) {
                   trace_line(row, idx as i16, &objects_clone)
                }
            });
        }
    });

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1023, 767);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image_data).unwrap(); // Save
}
